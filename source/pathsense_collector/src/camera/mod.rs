//
// This module defines a Camera struct and related functionality for handling
// camera operations in a multi-threaded environment. It provides the ability
// to start and stop camera capture, access the most recent camera frame.
//

mod test;

use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_ANY, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH};
use std::error::Error;
use std::process::Command;
use std::sync::{Arc, RwLock};
use tokio::sync::watch;

/// Struct representing a camera.
/// It manages the camera job on a green thread and provide APIs for
/// accessing and saving the most recent frame.
pub struct Camera {
    /// Watcher channel for the most recent frame
    frame_watcher: Option<watch::Receiver<Option<Mat>>>,
    /// Flag indicating whether the camera is running
    is_running: Arc<RwLock<bool>>,
}

impl Camera {
    pub fn new() -> Self {
        let is_running = Arc::new(RwLock::new(false));
        let frame_watcher = None;

        Self {
            frame_watcher,
            is_running,
        }
    }

    /// Start a camera thread
    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        // Set the is_running flag to true
        {
            let mut is_running = self.is_running.write().unwrap();
            *is_running = true;
        }

        // Initialize the camera
        let camera_index = self.find_mjpg_camera()?;
        let mut video_capture = VideoCapture::new(camera_index, CAP_ANY)?;
        video_capture.set(CAP_PROP_FRAME_WIDTH, 320.0)?;
        video_capture.set(CAP_PROP_FRAME_HEIGHT, 240.0)?;

        // Prepare the variables for the thread
        let is_running = Arc::clone(&self.is_running);
        let (frame_watch_tx, frame_watch_rx) = watch::channel(None);
        self.frame_watcher = Some(frame_watch_rx);

        // Start the camera thread
        tokio::task::spawn_blocking(move || {
            // Start the video capture loop
            loop {
                // Check if the thread should be aborted
                {
                    let is_running_lock = is_running.read().unwrap();
                    if !*is_running_lock {
                        video_capture.release().unwrap_or(());
                        break;
                    }
                }

                // Read the frame from the camera
                let mut frame = Mat::default();
                match video_capture.read(&mut frame) {
                    Ok(false) => continue,
                    Ok(true) => {
                        // Send the frame to the watcher
                        frame_watch_tx.send(Some(frame)).unwrap();
                    }
                    Err(e) => {
                        log::error!("Error reading frame from camera: {}", e);
                        let mut is_running_lock = is_running.write().unwrap();
                        *is_running_lock = false;
                        break;
                    }
                }

                // Sleep for 50ms
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        });

        Ok(())
    }

    /// Get the most recent frame from the camera
    /// If the frame is not available, this function will block until the next frame is available
    pub async fn get_last_frame(&mut self) -> Result<Mat, Box<dyn Error>> {
        // Get the frame watcher
        let frame_watcher = match self.frame_watcher.as_mut() {
            Some(frame_watcher) => frame_watcher,
            None => return Err("Frame watcher is not initialized".into()),
        };

        loop {
            // Wait for the frame to be updated
            frame_watcher.changed().await?;

            // Get the last frame and return it
            if let Some(last_frame) = frame_watcher.borrow().clone() {
                if !last_frame.empty() {
                    return Ok(last_frame.clone());
                }
            }
        }
    }

    /// Stop the camera thread
    pub fn stop(&mut self) {
        // Abort the camera thread
        let mut is_running = self.is_running.write().unwrap();
        *is_running = false;
    }

    /// Check whether the camera is running
    pub fn is_running(&self) -> bool {
        let is_running = self.is_running.read().unwrap();
        *is_running
    }

    /// Find the camera index that supports mjpg format
    fn find_mjpg_camera(&self) -> Result<i32, Box<dyn Error>> {
        for i in 0..10 {
            // Use v4l2 to check the formats of each camera
            let format_output = match Command::new("v4l2-ctl")
                .arg("--device")
                .arg(format!("/dev/video{i}"))
                .arg("--list-formats")
                .output()
            {
                Ok(output) => output,
                Err(_) => continue,
            };

            // If the the camera supports mjpg format, use it
            if format_output.status.success() {
                let formats = String::from_utf8(format_output.stdout).unwrap_or_default();
                if formats.contains("'MJPG'") {
                    return Ok(i);
                }
            }
        }
        Err("No camera supports MJPG format".into())
    }

    // Use v4l2 to set camera configurations
    // fn configure_camera(&self, camera_index: i32) -> Result<(), Box<dyn Error>> {
    //     let output = Command::new("v4l2-ctl")
    //         .arg("--device")
    //         .arg(format!("/dev/video{camera_index}"))
    //         .arg("--set-ctrl")
    //         .arg("auto_exposure=3")
    //         .output()?;
    //     if !output.status.success() {
    //         return Err("Failed to configure camera".into());
    //     }
    //     let output = Command::new("v4l2-ctl")
    //         .arg("--device")
    //         .arg(format!("/dev/video{camera_index}"))
    //         .arg("--set-ctrl")
    //         .arg("exposure_dynamic_framerate=0")
    //         .output()?;
    //     if !output.status.success() {
    //         return Err("Failed to configure camera".into());
    //     }
    //     Ok(())
    // }
}
