#[cfg(test)]
mod camera_test {
    use super::super::Camera;
    use opencv::prelude::*;
    use std::env;

    #[tokio::test]
    async fn full_test() {
        // Set the TMP_DIR_PATH environment variable
        env::set_var("TMP_DIR_PATH", "dependencies/tmp");

        // Delete the last frame file if exists
        tokio::fs::remove_file("dependencies/tmp/last_frame.jpg")
            .await
            .unwrap_or(());

        let mut camera = Camera::new();

        // Start the camera thread
        camera.start().unwrap();
        assert!(camera.is_running());

        // Read the last frame
        let last_frame = camera.get_last_frame().await.unwrap();
        assert!(!last_frame.empty());
        println!("Last frame: {:?}", last_frame);

        // Save the last frame
        camera.save_last_frame().await.unwrap();
        assert!(tokio::fs::try_exists("dependencies/tmp/last_frame.jpg")
            .await
            .unwrap_or(false));

        // Stop the camera thread
        camera.stop();
        assert!(!camera.is_running());
    }
}
