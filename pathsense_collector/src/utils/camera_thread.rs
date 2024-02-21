use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_ANY, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH};
use regex::Regex;
use std::process::Command;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub struct CameraThreader {
    pub cam: Arc<Mutex<VideoCapture>>,
    pub last_frame: Arc<(Mutex<Option<Mat>>, Condvar)>,
    pub running: Arc<Mutex<bool>>,
}

impl CameraThreader {
    pub fn new(index: i32) -> Result<Self, Box<dyn std::error::Error>> {
        let cam = Arc::new(Mutex::new(VideoCapture::new(index, CAP_ANY)?));
        cam.lock().unwrap().set(CAP_PROP_FRAME_WIDTH, 320.0)?;
        cam.lock().unwrap().set(CAP_PROP_FRAME_HEIGHT, 240.0)?;
        let last_frame = Arc::new((Mutex::new(None), Condvar::new()));
        let running = Arc::new(Mutex::new(false));

        Ok(Self {
            cam,
            last_frame,
            running,
        })
    }

    pub fn start(&self) {
        let cam = Arc::clone(&self.cam);
        let last_frame = Arc::clone(&self.last_frame);
        let running = Arc::clone(&self.running);
        *running.lock().unwrap() = true;
        thread::spawn(move || {
            while *running.lock().unwrap() {
                let mut cam_lock = cam.lock().unwrap();
                let mut frame = Mat::default();
                cam_lock.read(&mut frame).unwrap();
                let (last_frame_lock, cvar) = &*last_frame;
                let mut last_frame = last_frame_lock.lock().unwrap();
                *last_frame = Some(frame);
                cvar.notify_one();
            }
            cam.lock().unwrap().release().unwrap();
        });
    }

    pub fn read_last_frame(&self) -> Result<(Mat, bool), Box<dyn std::error::Error>> {
        let (last_frame_lock, cvar) = &*self.last_frame;
        let mut last_frame = last_frame_lock.lock().unwrap();
        while last_frame.is_none() {
            last_frame = cvar.wait(last_frame).unwrap();
        }
        Ok((last_frame.take().unwrap(), *self.running.lock().unwrap()))
    }

    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
    }
}

pub fn find_camera_device() -> Result<i32, Box<dyn std::error::Error>> {
    // Iterate through the first 10 video devices
    for i in 0..10 {
        // Run v4l2-ctl to find the formats supported by the device
        let output = Command::new("v4l2-ctl")
            .arg(format!("--device=/dev/video{}", i))
            .arg("--list-formats")
            .output()?;

        // Check if the device supports a streaming video format
        let re = Regex::new(r"'MJPG'")?;
        if re.is_match(std::str::from_utf8(&output.stdout)?) {
            return Ok(i);
        }
    }

    // If no suitable device is found, return an error
    Err("No suitable device found".into())
}
