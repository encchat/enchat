
use keyring::Entry;
use tauri::Runtime;

#[tauri::command]
pub async fn login<R: Runtime>(auth_url: &str, app: tauri::AppHandle<R>) -> Result<String, tauri::Error> {
    let (tx, rx) = std::sync::mpsc::channel::<String>();
    let redirect_uri = std::env::var("REDIRECT_URI").unwrap();
    let auth_window = tauri::WindowBuilder::new(&app, "auth", tauri::WindowUrl::External(auth_url.parse().unwrap()))
        .on_navigation(move |url: url::Url| {
            let domain = url.domain().unwrap().to_owned();
            if domain == redirect_uri {
                println!("Here!");
                tx.send(url.as_str().to_owned()).ok();
            }
            true
        })
        .build()?;
    let url = rx.recv().unwrap();
    auth_window.close()?;
    Ok(url)
}

#[tauri::command]
pub fn get_refresh_token() -> Option<String> {
    let key_entry = Entry::new("enchat", "refresh_token").ok()?;
    key_entry.get_password().ok()
}
#[tauri::command]
pub fn set_refresh_token(refresh_token: &str) {
    let key_entry = Entry::new("enchat", "refresh_token").unwrap();
    key_entry.set_password(refresh_token);
}

#[tauri::command]
pub async fn logout() {
    let key_entry = Entry::new("enchat", "refresh_token").unwrap();
    key_entry.delete_password();
}