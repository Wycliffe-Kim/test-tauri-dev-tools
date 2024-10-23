/*
 * Copyright (C) 2024 nota inc. All rights reserved.
 * This source code is the property of nota inc. and is protected by copyright law. THE RECEIPT OR POSSESSION OF  THIS SOURCE CODE AND/OR RELATED INFORMATION DOES NOT CONVEY OR IMPLY TRANSFER OF OWNERSHIP, COPYRIGHT OR ANY RIGHTS INCLUDING BUT NOT LIMITED TO REPRODUCE, DISCLOSE OR DISTRIBUTE ITS CONTENTS, OR TO MANUFACTURE, USE, OR SELL ANYTHING THAT IT  MAY DESCRIBE, IN WHOLE OR IN PART.
 * Unauthorized disclosure, reproduction, modification, distribution, or commercial use of all or any part of this source code without prior written consent from Nota Inc. is strictly prohibited.
 * This source code is provided "as is" and without any warranty, express or implied.
 * Nota Inc. shall not be liable for any damages arising from the use of this source code.
 * For inquiries regarding the use of this source code, please contact Nota Inc. at:
 * Email: contact@nota.ai
 */

use std::path::PathBuf;

use tauri::AppHandle;

#[macro_export]
macro_rules! project_root_path {
    () => {
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or(String::new())
    };
}

#[macro_export]
macro_rules! resolve_resource {
    ($path:expr, $app:expr) => {
        if cfg!(target_os = "windows") {
            PathBuf::from(format!(
                "{}\\{}",
                $app.path()
                    .resource_dir()
                    .unwrap_or_default()
                    .to_string_lossy(),
                $path
            ))
        } else {
            PathBuf::from(format!(
                "{}/{}",
                $app.path()
                    .resource_dir()
                    .unwrap_or_default()
                    .to_string_lossy(),
                $path
            ))
        }
    };
}

#[macro_export]
macro_rules! gstreamer_root_path {
    ($app:expr, $path:expr) => {
        if cfg!(debug_assertions) {
            if cfg!(target_os = "windows") {
                PathBuf::from(format!(
                    "{}{}",
                    std::env::var("GSTREAMER_1_0_ROOT_MSVC_X86_64").unwrap_or(String::new()),
                    String::from($path)
                ))
            } else if cfg!(target_os = "macos") {
                PathBuf::from(format!(
                    "/Library/Frameworks/Gstreamer.framework/Versions/Current/{}",
                    String::from($path)
                ))
            } else {
                PathBuf::default()
            }
        } else {
            if cfg!(target_os = "windows") {
                PathBuf::from(format!(
                    "{}\\assets\\{}",
                    $app.path_resolver()
                        .resource_dir()
                        .unwrap_or_default()
                        .to_string_lossy(),
                    $path
                ))
            } else if cfg!(target_os = "macos") {
                PathBuf::from(format!(
                    "{}/assets/{}",
                    $app.path_resolver()
                        .resource_dir()
                        .unwrap_or_default()
                        .to_string_lossy(),
                    $path
                ))
            } else {
                PathBuf::default()
            }
        }
    };

    ($app:expr, $windows:expr, $macos:expr) => {
        if cfg!(debug_assertions) {
            if cfg!(target_os = "windows") {
                PathBuf::from(format!(
                    "{}{}",
                    std::env::var("GSTREAMER_1_0_ROOT_MSVC_X86_64").unwrap_or(String::new()),
                    String::from($windows)
                ))
            } else if cfg!(target_os = "macos") {
                PathBuf::from(format!(
                    "/Library/Frameworks/Gstreamer.framework/Versions/Current/{}",
                    String::from($macos)
                ))
            } else {
                PathBuf::default()
            }
        } else {
            if cfg!(target_os = "windows") {
                PathBuf::from(format!(
                    "{}\\assets\\{}",
                    $app.path_resolver()
                        .resource_dir()
                        .unwrap_or_default()
                        .to_string_lossy(),
                    $windows
                ))
            } else if cfg!(target_os = "macos") {
                PathBuf::from(format!(
                    "{}/assets/{}",
                    $app.path_resolver()
                        .resource_dir()
                        .unwrap_or_default()
                        .to_string_lossy(),
                    $macos
                ))
            } else {
                PathBuf::default()
            }
        }
    };
}

pub fn get_gstreamer_launch_path(app: &AppHandle) -> PathBuf {
    gstreamer_root_path!(app, "bin\\gst-launch-1.0.exe", "bin/gst-launch-1.0")
}

pub fn get_gstramer_lib_path(app: &AppHandle) -> PathBuf {
    gstreamer_root_path!(app, "lib")
}

pub fn get_gstramer_plugin_path(app: &AppHandle) -> PathBuf {
    gstreamer_root_path!(app, "libexec\\gstreamer-1.0", "libexec/gstreamer-1.0")
}

pub fn get_gstramer_plugin_scanner_path(app: &AppHandle) -> PathBuf {
    gstreamer_root_path!(
        app,
        "libexec\\gstreamer-1.0\\gst-plugin-scanner.exe",
        "libexec/gstreamer-1.0/gst-plugin-scanner"
    )
}

pub fn get_hls_file_name(server_id: &str) -> String {
    format!("hls_play_{server_id}.m3u8")
}

pub fn get_output_dir(app: &AppHandle) -> PathBuf {
    app.path_resolver().app_data_dir().unwrap_or_default()
}

pub fn clear_files(output_dir: &PathBuf) -> std::io::Result<()> {
    for entry in std::fs::read_dir(output_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "ts" || ext == "m3u8" {
                    std::fs::remove_file(path.clone())?;
                }
            }
        }
    }

    Ok(())
}
