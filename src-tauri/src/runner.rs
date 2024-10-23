/*
 * Copyright (C) 2024 nota inc. All rights reserved.
 * This source code is the property of nota inc. and is protected by copyright law. THE RECEIPT OR POSSESSION OF  THIS SOURCE CODE AND/OR RELATED INFORMATION DOES NOT CONVEY OR IMPLY TRANSFER OF OWNERSHIP, COPYRIGHT OR ANY RIGHTS INCLUDING BUT NOT LIMITED TO REPRODUCE, DISCLOSE OR DISTRIBUTE ITS CONTENTS, OR TO MANUFACTURE, USE, OR SELL ANYTHING THAT IT  MAY DESCRIBE, IN WHOLE OR IN PART.
 * Unauthorized disclosure, reproduction, modification, distribution, or commercial use of all or any part of this source code without prior written consent from Nota Inc. is strictly prohibited.
 * This source code is provided "as is" and without any warranty, express or implied.
 * Nota Inc. shall not be liable for any damages arising from the use of this source code.
 * For inquiries regarding the use of this source code, please contact Nota Inc. at:
 * Email: contact@nota.ai
 */

use std::process::Stdio;

use tauri::AppHandle;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

use crate::{
    project_root_path,
    utils::{
        clear_files, get_gstramer_lib_path, get_gstramer_plugin_path,
        get_gstramer_plugin_scanner_path, get_gstreamer_launch_path, get_hls_file_name,
        get_location, get_output_dir, get_playlist_location, get_segment_location,
    },
};

pub async fn run(app: &AppHandle) -> Result<(), String> {
    let invoker = "run";
    log::info!("project_root_path: {}", project_root_path!());

    let output_dir = get_output_dir(app);
    let hls_file_name = get_hls_file_name("test");
    let src = output_dir
        .join(hls_file_name.clone())
        .to_str()
        .unwrap_or("")
        .to_string();

    if let Err(error) = clear_files(&get_output_dir(app)) {
        log::error!("{invoker} clear_files: {}", error.to_string());
    }

    let gstreamer_launch_path = get_gstreamer_launch_path(app);
    let gstreamer_lib_path = get_gstramer_lib_path(app);
    let gstreamer_plugin_path = get_gstramer_plugin_path(app);
    let gstreamer_plugin_scanner_path = get_gstramer_plugin_scanner_path(app);
    let location = get_location("rtsp://210.99.70.120:1935/live/cctv001.stream");
    let playlist_location = get_playlist_location(&src);
    let segment_location = get_segment_location(&output_dir);

    log::info!(
        "{invoker} gstreamer_launch_path: {}",
        gstreamer_launch_path.to_str().unwrap_or_default()
    );
    log::info!(
        "{invoker} gstreamer_lib_path: {}",
        gstreamer_lib_path.to_str().unwrap_or_default()
    );
    log::info!(
        "{invoker} gstreamer_plugin_path: {}",
        gstreamer_plugin_path.to_str().unwrap_or_default()
    );
    log::info!(
        "{invoker} gstreamer_plugin_scanner_path: {}",
        gstreamer_plugin_scanner_path.to_str().unwrap_or_default()
    );
    log::info!("{invoker} location: {location}");
    log::info!("{invoker} playlist_location: {playlist_location}");
    log::info!("{invoker} segment_location: {segment_location}");

    let mut process = match Command::new(gstreamer_launch_path)
        .args(&[
            "-v",
            "rtspsrc",
            &location,
            "!",
            "rtph264depay",
            "!",
            "h264parse",
            "!",
            "mpegtsmux",
            "!",
            "hlssink",
            "max-files=100",
            "target-duration=1",
            "playlist-length=100",
            &playlist_location,
            &segment_location,
        ])
        .env(
            "DYLD_LIBRARY_PATH",
            gstreamer_lib_path
                .to_str()
                .unwrap_or_default()
                .replace("\\", "\\\\"),
        )
        .env(
            "GST_PLUGIN_PATH",
            gstreamer_plugin_path
                .to_str()
                .unwrap_or_default()
                .replace("\\", "\\\\"),
        )
        .env(
            "GST_PLUGIN_SCANNER",
            gstreamer_plugin_scanner_path
                .to_str()
                .unwrap_or_default()
                .replace("\\", "\\\\"),
        )
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(process) => process,
        Err(error) => {
            log::error!("{invoker} Command::new {}", error.to_string());
            return Err(error.to_string());
        }
    };

    if let Some(stdout) = process.stdout.take() {
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                log::info!("{invoker} {}", line);
            }
        });
    }

    if let Some(stderr) = process.stderr.take() {
        tokio::spawn(async move {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                log::error!("{invoker} {}", line);
            }
        });
    }

    Ok(())
}
