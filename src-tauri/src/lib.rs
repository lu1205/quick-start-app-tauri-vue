// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use windows_icons::{get_icon_base64_by_path};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_file_icon(path: &str) -> String {
    generate_icon_from_file_type(path)
}

#[tauri::command]
fn get_shortcut_target(path: &str) -> Option<String> {
    resolve_shortcut(path)
}

#[tauri::command]
fn open_software(path: &str) -> Result<(), String> {
    use winapi::um::shellapi::{ShellExecuteW};
    use winapi::um::winuser::{SW_SHOW};
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr;
    
    // 将路径转换为宽字符串
    let path_wide: Vec<u16> = OsStr::new(path)
        .encode_wide()
        .chain(Some(0))
        .collect();
    
    // 使用ShellExecute打开软件
    // SW_SHOW会激活并显示窗口，如果窗口最小化或最大化，会恢复到原始大小
    let result = unsafe {
        ShellExecuteW(
            ptr::null_mut(), // 父窗口句柄
            ptr::null(),     // 操作（默认为"open"）
            path_wide.as_ptr(), // 要打开的文件路径
            ptr::null(),     // 命令行参数
            ptr::null(),     // 工作目录
            SW_SHOW          // 显示方式
        )
    };
    
    // ShellExecute返回大于32的值表示成功
    let result_code = result as i32;
    if result_code > 32 {
        Ok(())
    } else {
        Err(format!("Failed to open software: ShellExecute failed with code {}", result_code))
    }
}


// 基于文件类型生成图标的辅助函数
fn generate_icon_from_file_type(path: &str) -> String {
    // 检查是否为.lnk文件
    let path_str = path.to_string();
    if path_str.ends_with(".lnk") {
        // 尝试解析.lnk文件，获取目标路径
        if let Some(target_path) = resolve_shortcut(path) {
            // 使用目标路径获取图标
            match get_icon_base64_by_path(&target_path) {
                Ok(icon) => {
                    return format!("data:image/png;base64,{}", icon);
                }
                Err(_) => {
                    // 如果获取目标文件图标失败，尝试使用原始路径
                }
            }
        }
    }
    
    // 尝试使用windows_icons获取图标
    match get_icon_base64_by_path(path) {
        Ok(icon) => {
            format!("data:image/png;base64,{}", icon)
        }
        Err(_) => {
            // 返回空字符串
            "".to_string()
        }
    }
}

// 解析Windows快捷方式(.lnk文件)，返回目标路径
#[cfg(windows)]
fn resolve_shortcut(lnk_path: &str) -> Option<String> {
    use std::process::Command;
    
    // 使用PowerShell命令解析lnk文件，添加-WindowStyle Hidden参数隐藏窗口
    let output = Command::new("powershell")
        .arg("-WindowStyle")
        .arg("Hidden")
        .arg("-Command")
        .arg(format!("(New-Object -COM WScript.Shell).CreateShortcut('{}').TargetPath", lnk_path))
        .output()
        .ok()?;
    
    if output.status.success() {
        let target_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !target_path.is_empty() {
            return Some(target_path);
        }
    }
    
    None
}

// 非Windows平台的默认实现
#[cfg(not(windows))]
fn resolve_shortcut(_lnk_path: &str) -> Option<String> {
    None
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![greet, get_file_icon, open_software, get_shortcut_target])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
