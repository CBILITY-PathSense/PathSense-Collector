import threading
import numpy as np
from typing import Tuple, Union
import cv2
from cv2.typing import MatLike

class CameraThreader(threading.Thread):
    def __init__(self, cap):
        super().__init__()
        self.cap:cv2.VideoCapture = cap
        self.last_frame:Union[MatLike,None] = None
        self.running = True
        self.frame_ready = threading.Condition()

    def run(self):
        while self.running:
            ret, frame = self.cap.read()
            if ret:
                if frame is not None and frame.size > 0:
                    with self.frame_ready:
                        self.last_frame = frame
                        self.frame_ready.notify()
            else:
                self.cap.release()
                self.running = False

    def read_last_frame(self) -> Tuple[bool, Union[MatLike,None]]:
        with self.frame_ready:
            self.frame_ready.wait()
            return self.running, self.last_frame

    def stop(self):
        self.cap.release()
        self.running = False
