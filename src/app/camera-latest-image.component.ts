import { Component, OnInit, Input } from "@angular/core";
import { CameraService } from "./camera.service";
import { Camera } from "./camera";

@Component({
  selector: "camera-latest-image",
  templateUrl: "./camera-latest-image.component.html",
})
export class CameraLatestImageComponent implements OnInit {
  @Input() name: string;
  camera: Camera;

  constructor(private cameraService: CameraService) { }

  ngOnInit(): void {
    this.cameraService.getCamera(this.name).then(camera => this.camera = camera);
  }
}
