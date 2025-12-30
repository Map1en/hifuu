use std::fs;
use walkdir::WalkDir;

#[tauri::command]
pub fn get_all_screenshot(path: &str) -> Result<(), String> {
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            println!("file:{}", path.display())
        }
    }
    Ok(())
}
