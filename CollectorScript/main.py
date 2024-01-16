import cv2
import time
import os
from datetime import datetime
import subprocess
import re
from dependencies.camera_threading import CameraThreader

def main():
    username = os.environ.get('SUDO_USER')
    target_directory = os.path.join(os.path.expanduser(f'~{username}'), f"pathsense_images_{datetime.now().strftime('%Y-%m-%d')}")
    os.makedirs(target_directory, exist_ok=True)
    cam_thread = None
    cap = None

    # Keep alive until forced to close
    try:
        while True:
            # Find and open camera
            cap = None
            camera = find_camera_device()
            if camera is None:
                print("No camera found.")
                break
            else:
                try:
                    cap = cv2.VideoCapture(camera)
                    print(f"Opened camera at camera number {camera}")
                except Exception as e:
                    print(e)
                    break

            # Set camera resolution
            cap.set(cv2.CAP_PROP_FRAME_WIDTH, 320)
            cap.set(cv2.CAP_PROP_FRAME_HEIGHT, 240)

            #start camera threader
            cam_thread = CameraThreader(cap)
            cam_thread.start()

            try:
                while True:
                    # Capture image
                    running, frame = cam_thread.read_last_frame()
                    if not running: 
                        break

                    # Save image to target directory as a .jpg file
                    filename = os.path.join(target_directory, f"{datetime.now().strftime('%H-%M-%S')}.jpg")
                    if frame is not None:
                        cv2.imwrite(filename, frame)
                    print(f"Image Saved: {filename}")

                    # Delay before capturing next image
                    time.sleep(2)
            
            # If forced to close, close camera and exit
            except KeyboardInterrupt:
                print("Keyboard interrupted")
                break
            
            # If error occurs, try again
            except Exception as e:
                print(e)

            # Close camera before trying again in 5 seconds
            finally:
                if cap != None:
                    cap.release()
                if cam_thread != None:
                    cam_thread.stop()
                cap = None
                time.sleep(5)
        
    except Exception as e:
        print(e)

    # If forced to close, close camera and exit permanently
    finally:
        if cap != None:
            cap.release()
        if cam_thread != None: 
            cam_thread.stop()

def find_camera_device():
    # Iterate through the first 10 video devices
    for i in range(10):
        try:
            # Run v4l2-ctl to find the formats supported by the device
            output = subprocess.check_output(["v4l2-ctl", "--device=/dev/video" + str(i), "--list-formats"])
        except subprocess.CalledProcessError:
            # If the device does not exist, skip to the next one
            continue

        # Check if the device supports a streaming video format (e.g., 'YUYV', 'MJPG', 'H264')
        if re.search(b'\'MJPG\'', output):
            return "/dev/video" + str(i)

    # If no suitable device is found, return None
    return None

if __name__ == "__main__":
    main()

