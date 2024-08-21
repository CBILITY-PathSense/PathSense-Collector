mod test;

use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_ANY, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH};
use std::env;
use std::error::Error;
use std::process::Command;
use std::sync::{Arc, RwLock};
use tokio::fs::remove_file;
use tokio::sync::watch;

/// Struct representing a camera.
/// It manages the camera job on a green thread and provide APIs for
/// accessing and saving the most recent frame.
pub struct Camera {
    frame_watcher: Option<watch::Receiver<Option<Mat>>>,
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
        let mut video_capture = VideoCapture::new(self.find_mjpg_camera()?, CAP_ANY)?;
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
                        continue;
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

    /// Save the most recent frame to a file in the temp directory set by TMP_DIR_PATH environment variable.
    /// If the frame is not available, this function will block until the next frame is available
    pub async fn save_last_frame(&mut self) -> Result<(), Box<dyn Error>> {
        // Get the path to asave the last frame
        let path = env::var("TMP_DIR_PATH").expect("TMP_DIR_PATH not set");
        let path = format!("{}/last_frame.jpg", path);

        // Get the last frame
        let frame = self.get_last_frame().await?;

        //Delete tmp file if exists
        remove_file(&path).await.unwrap_or(());

        // Safe mat to file
        opencv::imgcodecs::imwrite(&path, &frame, &opencv::core::Vector::new())?;

        Ok(())
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
}
