import { Component } from "@angular/core";
import { CameraService } from "./camera.service";
import { Camera } from "./camera";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.css"],
  providers: [CameraService],
})
export class AppComponent {
  title = "ATLAS";
  cameras: Camera[];
  selectedCamera: Camera;

  constructor(private cameraService: CameraService) { }

  getCameras(): void {
    this.cameraService.getCameras().then(cameras => this.cameras = cameras);
  }

  ngOnInit(): void {
    this.getCameras();
  }

  onSelect(camera: Camera): void {
    this.selectedCamera = camera;
  }
}
