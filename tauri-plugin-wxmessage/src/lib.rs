use config::Config;
use reqwest;
use std::{
    collections::HashMap,
    fs::{File, Permissions},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use tauri::{
    plugin::{Builder, TauriPlugin},
    utils::platform::current_exe,
    AppHandle, Manager, RunEvent, Runtime, Url,
};
mod commands;
mod config;
mod error;
mod models;
use crate::models::ServerVersion;
pub use error::{Error, Result};
use std::io::copy;
type ChildStore = Arc<Mutex<Option<Child>>>;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the wxmessage APIs.
pub trait WxmessageExt<R: Runtime> {
    fn wxmessage(&self) -> &Wxmessage<R>;
}

impl<R: Runtime, T: Manager<R>> crate::WxmessageExt<R> for T {
    fn wxmessage(&self) -> &Wxmessage<R> {
        self.state::<Wxmessage<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R, Config> {
    Builder::<R, Config>::new("wxmessage")
        .invoke_handler(tauri::generate_handler![
            commands::is_enabled,
            commands::disable,
            commands::enable
        ])
        .setup(move |app, api| {
            let config = api.config().clone();

            let wxmessage = Wxmessage {
                config,
                is_running: Arc::new(AtomicBool::new(false)),
                app: app.clone(),
                ps: ChildStore::default(),
            };

            app.manage(wxmessage);

            background_task(app.clone());

            Ok(())
        })
        .on_event(|app, event| {
            if let RunEvent::Exit = event {
                let wx = app.state::<Wxmessage<R>>();
                let children = {
                    let mut lock = wx.ps.lock().unwrap();
                    std::mem::take(&mut *lock)
                };

                if let Some(mut child) = children {
                    let _ = child.kill();
                }
            }
        })
        .build()
}

/// Access to the wxmessage APIs.
pub struct Wxmessage<R: Runtime> {
    is_running: Arc<AtomicBool>,
    config: Config,
    app: AppHandle<R>,
    ps: ChildStore,
}

impl<R: Runtime> Wxmessage<R> {
    pub fn is_enabled(&self) -> crate::Result<bool> {
        Ok(self.is_running.load(Ordering::Relaxed))
    }

    pub fn disable(&self) -> crate::Result<()> {
        self.is_running.store(false, Ordering::Relaxed);
        Ok(())
    }

    pub async fn enable(&self, args: Vec<String>) -> crate::Result<()> {
        if self.is_running.load(Ordering::Relaxed) {
            return Ok(()); // 避免重复启动
        }

        self.is_running.store(true, Ordering::Relaxed);

        let executable_path = current_exe()?;
        // Get the extract_path from the provided executable_path
        let mut plugins_path = if cfg!(target_os = "linux") {
            executable_path
        } else {
            extract_path_from_executable(&executable_path)?
        };

        plugins_path.push("plugins");

        println!("{}", plugins_path.display());

        let endpoints = self.config.endpoints.clone();

        let server_version = get_server_version(endpoints).await?;

        if server_version.is_none() {
            self.is_running.store(false, Ordering::Relaxed);
            println!("No update available");
            return Ok(());
        }

        let server_version = server_version.unwrap();

        if !plugins_path.exists() {
            std::fs::create_dir_all(&plugins_path)?;
        }

        #[cfg(target_os = "windows")]
        plugins_path.push(format!("wxmessage-{}.exe", server_version.version));
        #[cfg(target_os = "macos")]
        plugins_path.push(format!("wxmessage-{}", server_version.version));

        if !plugins_path.exists() {
            let rs = download(&server_version.download_url, &plugins_path).await;
            if rs.is_err() {
                println!("download failed");
                self.is_running.store(false, Ordering::Relaxed);
                return Ok(());
            }
        }

        let mut command = Command::new(plugins_path);
        command.args(&args);
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());
        // 启动子进程
        let child = command.spawn()?;
        let mut guard = self.ps.lock().unwrap();
        *guard = Some(child);
        Ok(())
    }

    pub async fn check_update(&self) -> crate::Result<bool> {
        let endpoints = self.config.endpoints.clone();
        let server_version = get_server_version(endpoints).await?;
        if server_version.is_none() {
            return Ok(false);
        }

        let executable_path = current_exe()?;
        // Get the extract_path from the provided executable_path
        let mut cli_path = if cfg!(target_os = "linux") {
            executable_path
        } else {
            extract_path_from_executable(&executable_path)?
        };

        cli_path.push("plugins");

        let sv = server_version.unwrap();

        #[cfg(target_os = "windows")]
        cli_path.push(format!("wxmessage-{}.exe", sv.version));
        #[cfg(target_os = "macos")]
        cli_path.push(format!("wxmessage-{}", sv.version));

        if cli_path.exists() {
            return Ok(true);
        }
        Ok(false)
    }
}

fn background_task<R: Runtime>(app: AppHandle<R>) {
    let handle = app.clone();
    std::thread::spawn(move || loop {
        let wx = handle.state::<Wxmessage<R>>();
        let is_running = wx.is_enabled().unwrap();

        if is_running {
            let mut guard = wx.ps.lock().unwrap();
            if let Some(ref mut child) = *guard {
                match child.try_wait() {
                    Ok(Some(_status)) => {
                        wx.is_running.store(false, Ordering::Relaxed);
                        *guard = None;
                    }
                    Ok(None) => {}
                    Err(_) => {
                        wx.is_running.store(false, Ordering::Relaxed);
                        *guard = None;
                    }
                }
            }
        } else {
            let mut guard = wx.ps.lock().unwrap();
            if let Some(ref mut child) = *guard {
                let _ = child.kill();
                *guard = None;
            }
        }

        std::thread::sleep(Duration::from_secs(1));
    });
}

pub fn extract_path_from_executable(executable_path: &Path) -> Result<PathBuf> {
    // Return the path of the current executable by default
    // Example C:\Program Files\My App\
    let extract_path = executable_path
        .parent()
        .map(PathBuf::from)
        .ok_or(Error::FailedToDetermineExtractPath)?;

    // MacOS example binary is in /Applications/TestApp.app/Contents/MacOS/myApp
    // We need to get /Applications/<app>.app
    // TODO(lemarier): Need a better way here
    // Maybe we could search for <*.app> to get the right path
    #[cfg(target_os = "macos")]
    if extract_path
        .display()
        .to_string()
        .contains("Contents/MacOS")
    {
        return extract_path
            .parent()
            .map(PathBuf::from)
            .ok_or(Error::FailedToDetermineExtractPath)?
            .parent()
            .map(PathBuf::from)
            .ok_or(Error::FailedToDetermineExtractPath);
    }

    Ok(extract_path)
}

/// Gets the target string used on the updater.
pub fn target() -> Option<String> {
    if let (Some(target), Some(arch)) = (get_updater_target(), get_updater_arch()) {
        Some(format!("{target}-{arch}"))
    } else {
        None
    }
}

pub fn get_updater_target() -> Option<String> {
    if cfg!(target_os = "linux") {
        Some("linux".to_owned())
    } else if cfg!(target_os = "macos") {
        // TODO shouldn't this be macos instead?
        Some("darwin".to_owned())
    } else if cfg!(target_os = "windows") {
        Some("windows".to_owned())
    } else {
        None
    }
}

pub fn get_updater_arch() -> Option<String> {
    if cfg!(target_arch = "x86") {
        Some("i686".to_owned())
    } else if cfg!(target_arch = "x86_64") {
        Some("x86_64".to_owned())
    } else if cfg!(target_arch = "arm") {
        Some("armv7".to_owned())
    } else if cfg!(target_arch = "aarch64") {
        Some("aarch64".to_owned())
    } else if cfg!(target_arch = "riscv64") {
        Some("riscv64".to_owned())
    } else {
        None
    }
}

async fn download(url: &String, target: &Path) -> crate::Result<()> {
    let response = reqwest::get(url).await?;
    // 检查请求是否成功
    if response.status().is_success() {
        // 创建文件并写入内容
        let mut file = File::create(target)?;

        let content = response.bytes().await?;

        copy(&mut content.as_ref(), &mut file)?;

        #[cfg(target_os = "macos")]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = Permissions::from_mode(0755);
            std::fs::set_permissions(target, permissions)?;
        }
    }
    Ok(())
}

async fn get_server_version(endpoints: Vec<Url>) -> crate::Result<Option<ServerVersion>> {
    let version = "1.0.0";

    let target = get_updater_target().unwrap();
    let arch = get_updater_arch().unwrap();

    for url in &endpoints {
        let url: Url = url
            .to_string()
            // url::Url automatically url-encodes the path components
            .replace("%7B%7Bcurrent_version%7D%7D", &version)
            .replace("%7B%7Btarget%7D%7D", &target)
            .replace("%7B%7Barch%7D%7D", &arch)
            // but not query parameters
            .replace("{{current_version}}", &version)
            .replace("{{target}}", &target)
            .replace("{{arch}}", &arch)
            .parse()?;

        println!("{}", url);
        let response = reqwest::get(url).await?;
        if response.status().is_success() {
            let body = response.json::<HashMap<String, String>>().await?;
            if body.is_empty() {
                continue;
            }

            let sv = ServerVersion {
                version: body.get("version").unwrap().to_string(),
                download_url: body.get("download_url").unwrap().to_string(),
            };
            return Ok(Some(sv));
        }
    }

    Ok(None)
}
