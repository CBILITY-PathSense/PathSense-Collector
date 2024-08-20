mod utils;

use opencv::core::Vector;
use opencv::imgcodecs;
use rand::Rng;
use std::fs;
use tokio::time;
use utils::camera_thread::{find_camera_device, CameraThreader};

// Assuming find_camera_device and CameraThreader are defined elsewhere

#[tokio::main]
async fn main() -> ! {
    let home_dir = dirs::home_dir().unwrap();
    let target_directory = home_dir.join(format!(
        "pathsense_images_{}",
        rand::thread_rng().gen_range(1000..9999)
    ));
    fs::create_dir_all(&target_directory).unwrap();

    let mut num = 0;

    loop {
        let cam_thread = match CameraThreader::new(find_camera_device().unwrap_or(0)) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error: {}", e);
                time::sleep(time::Duration::from_secs(2)).await;
                continue;
            }
        };
        cam_thread.start();

        loop {
            let (frame, running) = match cam_thread.read_last_frame() {
                Ok((frame, running)) => (frame, running),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    break;
                }
            };
            if !running {
                eprintln!("Camera failed to start");
                break;
            }

            let filename = target_directory.join(format!(
                "{}_{}.jpg",
                num,
                rand::thread_rng().gen_range(1000..9999)
            ));

            // Save the frame to a file
            imgcodecs::imwrite(
                filename.to_str().unwrap_or("/error.jpg"),
                &frame,
                &Vector::new(),
            )
            .unwrap();
            println!("Image Saved: {:?}", filename);

            num += 1;
            time::sleep(time::Duration::from_secs(2)).await
        }
        cam_thread.stop();
        time::sleep(time::Duration::from_secs(5)).await
    }
}
