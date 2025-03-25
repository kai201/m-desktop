use std::io::copy;
use std::{
    collections::HashMap,
    fs::{self, create_dir_all, File, Permissions},
};

#[cfg(target_os = "macos")]
use std::os::unix::fs::PermissionsExt;

fn set_executable_permission(file_path: &str) {
    // 设置文件权限为可执行

    #[cfg(target_os = "macos")]
    let permissions = Permissions::from_mode(0755); // rwxr-xr-x

    #[cfg(target_os = "macos")]
    let _ = fs::set_permissions(file_path, permissions);
}

pub async fn get_json(url: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    // 设置请求头
    let response = client
        .get(url) // 目标 URL
        .header("os", std::env::consts::OS.to_string()) // 设置 User-Agent
        .header("arch", std::env::consts::ARCH.to_string()) // 设置 Authorization
        .send()
        .await?;

    // 检查请求是否成功
    if response.status().is_success() {
        // 解析JSON响应
        let json = response.json::<HashMap<String, String>>().await?;
        Ok(json)
    } else {
        Err(format!("请求失败: {}", response.status()).into())
    }
}

pub async fn download(url: &str, target_path: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    // 检查请求是否成功
    if response.status().is_success() {
        // 创建目标目录（如果不存在）
        if let Some(parent) = std::path::Path::new(target_path).parent() {
            if !parent.exists() {
                create_dir_all(parent)?;
            }
        }

        // 创建文件并写入内容
        let mut file = File::create(target_path)?;

        let content = response.bytes().await?;

        copy(&mut content.as_ref(), &mut file)?;

        set_executable_permission(&target_path);
        Ok(true)
    } else {
        Err(format!("下载文件失败: {}", response.status()).into())
    }
}
