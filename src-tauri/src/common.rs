use serde::{Serialize, Deserialize};
use tauri::{Manager, State};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IconInfo {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub format: String,
}

pub struct IconManager {
    // 可以在这里添加图标缓存
}

impl IconManager {
    pub fn new() -> Self {
        IconManager {}
    }
    
    pub fn get_executable_icon(&self, path: Option<&std::path::Path>) -> Result<IconInfo, Box<dyn std::error::Error>> {
        // 如果提供了路径，使用提供的路径；否则使用当前可执行文件路径
        let target_path = path.unwrap_or_else(|| std::env::current_exe().unwrap());
        
        #[cfg(target_os = "windows")]
        {
            return self.get_windows_exe_icon(&target_path);
        }
        
        #[cfg(target_os = "macos")]
        {
            return self.get_macos_app_icon();
        }
        
        #[cfg(target_os = "linux")]
        {
            return self.get_linux_app_icon();
        }
        
        // 默认返回空图标
        Ok(IconInfo {
            width: 0,
            height: 0,
            data: vec![],
            format: "".to_string(),
        })
    }
    
    #[cfg(target_os = "windows")]
    fn get_windows_exe_icon(&self, exe_path: &std::path::Path) -> Result<IconInfo, Box<dyn std::error::Error>> {
        use winapi::um::shellapi::{ExtractIconExW, DestroyIcon};
        use winapi::um::winuser::{GetIconInfo};
        use winapi::um::wingdi::{CreateCompatibleDC, CreateCompatibleBitmap, SelectObject, DeleteDC, DeleteObject, GetDIBits, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, RGBQUAD};
        use winapi::shared::windef::{HICON, HBITMAP, HDC};
        use winapi::um::winuser::ICONINFO;
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use std::mem;
        use std::ptr;
        use std::vec::Vec;
        
        unsafe {
            let path: Vec<u16> = exe_path
                .as_os_str()
                .encode_wide()
                .chain(Some(0))
                .collect();
            
            let mut large_icon: HICON = ptr::null_mut();
            let mut small_icon: HICON = ptr::null_mut();
            
            let count = ExtractIconExW(
                path.as_ptr(),
                0,
                &mut large_icon,
                &mut small_icon,
                1
            );
            
            if count > 0 {
                // 使用大图标
                let icon_to_use = if !large_icon.is_null() {
                    large_icon
                } else if !small_icon.is_null() {
                    small_icon
                } else {
                    return Err("No icon found".into());
                };
                
                // 获取图标信息
                let mut icon_info = ICONINFO {
                    fIcon: 0,
                    xHotspot: 0,
                    yHotspot: 0,
                    hbmMask: ptr::null_mut(),
                    hbmColor: ptr::null_mut(),
                };
                
                if GetIconInfo(icon_to_use, &mut icon_info) == 0 {
                    if !large_icon.is_null() {
                        DestroyIcon(large_icon);
                    }
                    if !small_icon.is_null() {
                        DestroyIcon(small_icon);
                    }
                    return Err("Failed to get icon info".into());
                }
                
                // 创建兼容DC
                let hdc: HDC = CreateCompatibleDC(ptr::null_mut());
                if hdc.is_null() {
                    if !large_icon.is_null() {
                        DestroyIcon(large_icon);
                    }
                    if !small_icon.is_null() {
                        DestroyIcon(small_icon);
                    }
                    if !icon_info.hbmColor.is_null() {
                        DeleteObject(icon_info.hbmColor as _);
                    }
                    if !icon_info.hbmMask.is_null() {
                        DeleteObject(icon_info.hbmMask as _);
                    }
                    return Err("Failed to create compatible DC".into());
                }
                
                // 图标大小
                const ICON_SIZE: u32 = 32;
                
                // 创建位图信息
                let mut bmp_info = BITMAPINFO {
                    bmiHeader: BITMAPINFOHEADER {
                        biSize: mem::size_of::<BITMAPINFOHEADER>() as _,
                        biWidth: ICON_SIZE as _,
                        biHeight: -(ICON_SIZE as _), // 负数值表示从上到下
                        biPlanes: 1,
                        biBitCount: 32,
                        biCompression: BI_RGB as _,
                        biSizeImage: 0,
                        biXPelsPerMeter: 0,
                        biYPelsPerMeter: 0,
                        biClrUsed: 0,
                        biClrImportant: 0,
                    },
                    bmiColors: [RGBQUAD { rgbBlue: 0, rgbGreen: 0, rgbRed: 0, rgbReserved: 0 }; 1],
                };
                
                // 创建兼容位图
                let hbitmap: HBITMAP = CreateCompatibleBitmap(hdc, ICON_SIZE as _, ICON_SIZE as _);
                if hbitmap.is_null() {
                    DeleteDC(hdc);
                    if !large_icon.is_null() {
                        DestroyIcon(large_icon);
                    }
                    if !small_icon.is_null() {
                        DestroyIcon(small_icon);
                    }
                    if !icon_info.hbmColor.is_null() {
                        DeleteObject(icon_info.hbmColor as _);
                    }
                    if !icon_info.hbmMask.is_null() {
                        DeleteObject(icon_info.hbmMask as _);
                    }
                    return Err("Failed to create compatible bitmap".into());
                }
                
                // 选择位图到DC
                let old_bitmap = SelectObject(hdc, hbitmap as _);
                
                // 准备存储图标数据
                let mut pixels: Vec<u8> = vec![0; (ICON_SIZE * ICON_SIZE * 4) as _];
                
                // 获取位图数据
                GetDIBits(
                    hdc,
                    icon_info.hbmColor,
                    0,
                    ICON_SIZE as _,
                    pixels.as_mut_ptr() as *mut _,
                    &mut bmp_info,
                    DIB_RGB_COLORS as _,
                );
                
                // 清理资源
                SelectObject(hdc, old_bitmap);
                DeleteObject(hbitmap as _);
                DeleteDC(hdc);
                
                if !large_icon.is_null() {
                    DestroyIcon(large_icon);
                }
                if !small_icon.is_null() {
                    DestroyIcon(small_icon);
                }
                if !icon_info.hbmColor.is_null() {
                    DeleteObject(icon_info.hbmColor as _);
                }
                if !icon_info.hbmMask.is_null() {
                    DeleteObject(icon_info.hbmMask as _);
                }
                
                // 返回图标信息
                Ok(IconInfo {
                    width: ICON_SIZE,
                    height: ICON_SIZE,
                    data: pixels,
                    format: "rgba".to_string(),
                })
            } else {
                Err("No icon found".into())
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    fn get_macos_app_icon(&self) -> Result<IconInfo, Box<dyn std::error::Error>> {
        // macOS 实现
        Ok(IconInfo {
            width: 32,
            height: 32,
            data: vec![],
            format: "png".to_string(),
        })
    }
    
    #[cfg(target_os = "linux")]
    fn get_linux_app_icon(&self) -> Result<IconInfo, Box<dyn std::error::Error>> {
        // Linux 实现
        Ok(IconInfo {
            width: 32,
            height: 32,
            data: vec![],
            format: "png".to_string(),
        })
    }
}

// Tauri 命令
#[tauri::command]
pub async fn get_application_icon(
    icon_manager: State<'_, Mutex<IconManager>>,
) -> Result<IconInfo, String> {
    let manager = icon_manager.lock().map_err(|e| e.to_string())?;
    manager.get_executable_icon(None)
        .map_err(|e| e.to_string())
}