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
    // 尝试使用windows_icons获取图标
    match get_icon_base64_by_path(path) {
        Ok(icon) => {
            format!("data:image/png;base64,{}", icon)
        }
        Err(_) => {
            // 注释掉备选方案代码
            // // 出错时使用基于文件类型的图标作为备选
            // let file_name = path.split('\\').last().unwrap_or("unknown");
            // let file_ext = file_name.split('.').last().unwrap_or("").to_lowercase();
            // 
            // // 根据文件扩展名生成不同的图标提示词
            // let icon_prompt = match file_ext.as_str() {
            //     "exe" => "executable file icon windows application",
            //     "lnk" => "shortcut file icon link arrow",
            //     "pdf" => "PDF document file icon red",
            //     "doc" | "docx" => "Word document file icon blue",
            //     "xls" | "xlsx" => "Excel spreadsheet file icon green",
            //     "ppt" | "pptx" => "PowerPoint presentation file icon orange",
            //     "txt" => "text file icon white paper",
            //     "jpg" | "jpeg" | "png" | "gif" => "image file icon colorful picture",
            //     "mp3" | "wav" | "flac" => "audio file icon music note",
            //     "mp4" | "avi" | "mkv" => "video file icon movie clapper",
            //     "zip" | "rar" | "7z" => "compressed file icon zip folder",
            //     _ => "generic file icon document",
            // };
            // 
            // // 生成基于文件类型的图标URL
            // format!("https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt={}%20colorful%20modern%20design&image_size=square", urlencoding::encode(icon_prompt))
            
            // 返回空字符串
            "".to_string()
        }
    }
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet, get_file_icon, open_software])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
