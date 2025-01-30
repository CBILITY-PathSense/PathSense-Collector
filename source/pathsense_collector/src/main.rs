mod camera;

use camera::Camera;
use opencv::core::Vector;
use opencv::imgcodecs;
use rand::Rng;
use std::fs;
use tokio::time;

#[tokio::main]
async fn main() -> ! {
    env_logger::init();

    let mut rng = rand::thread_rng();
    let target_directory = dirs::home_dir().unwrap().join(format!(
        "pathsense_images_{}",
        rng.gen_range(100000..999999)
    ));
    fs::create_dir_all(&target_directory).unwrap();

    let mut image_num = 0;

    loop {
        let mut cam = Camera::new();
        if let Err(e) = cam.start() {
            log::error!("Error starting camera thread: {}", e);
            time::sleep(time::Duration::from_secs(3)).await;
            continue;
        }

        loop {
            // Check if the camera thread is running
            if !cam.is_running() {
                break;
            }

            // Get the last frame from the camera
            let frame = match cam.get_last_frame().await {
                Ok(frame) => frame,
                Err(e) => {
                    log::error!("Error: {}", e);
                    break;
                }
            };

            // Generate a random filename
            let filename = target_directory.join(format!("image_{}.jpg", image_num));
            let filename = filename.to_str().unwrap();

            // Save the frame to a file
            match imgcodecs::imwrite(filename, &frame, &Vector::new()) {
                Ok(_) => log::info!("Image Saved: {:?}", filename),
                Err(e) => {
                    log::error!("Error saving image: {}", e);
                    break;
                }
            };

            image_num += 1;
            time::sleep(time::Duration::from_millis(1500)).await
        }

        cam.stop();
        time::sleep(time::Duration::from_secs(5)).await
    }
}
