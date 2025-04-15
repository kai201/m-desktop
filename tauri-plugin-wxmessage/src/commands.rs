use tauri::{command, AppHandle, Runtime};

use crate::Result;
use crate::WxmessageExt;

#[command]
pub(crate) async fn is_enabled<R: Runtime>(app: AppHandle<R>) -> Result<bool> {
    app.wxmessage().is_enabled()
}

#[command]
pub(crate) async fn disable<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    app.wxmessage().disable()
}

#[command]
pub(crate) async fn enable<R: Runtime>(app: AppHandle<R>, args: Vec<String>) -> Result<()> {
    println!("enable: {:?}", args);
    app.wxmessage().enable(args).await
}
