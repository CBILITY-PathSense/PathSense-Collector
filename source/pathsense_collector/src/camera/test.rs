#[cfg(test)]
mod camera_test {
    use super::super::Camera;
    use opencv::prelude::*;

    #[tokio::test]
    async fn full_test() {
        let mut camera = Camera::new();

        // Start the camera thread
        camera.start().unwrap();
        assert!(camera.is_running());

        // Read the last frame
        let last_frame = camera.get_last_frame().await.unwrap();
        assert!(!last_frame.empty());

        // Stop the camera thread
        camera.stop();
        assert!(!camera.is_running());
    }
}
