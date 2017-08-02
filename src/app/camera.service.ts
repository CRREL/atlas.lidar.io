import { Injectable } from "@angular/core";
import { Camera } from "./camera";
import { CAMERAS } from "./mock-cameras";

@Injectable()
export class CameraService {
  getCameras(): Promise<Camera[]> {
    return Promise.resolve(CAMERAS);
  }
}
