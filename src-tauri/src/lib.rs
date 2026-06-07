use tauri_plugin_opener::OpenerExt;

fn url_encode(s: &str) -> String {
    s.as_bytes().iter().map(|&b| match b {
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (b as char).to_string(),
        b' ' => "+".to_string(),
        _ => format!("%{:02X}", b),
    }).collect()
}

#[tauri::command]
fn open_baidu(app: tauri::AppHandle, keyword: String) {
    let url = format!("https://www.baidu.com/s?wd={}", url_encode(&keyword));
    let _ = app.opener().open_url(url, None::<&str>);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![open_baidu])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
