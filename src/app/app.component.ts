import { Component, OnInit } from "@angular/core";

import { Camera } from "./camera";
import { CameraService } from "./camera.service";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.css"],
})
export class AppComponent implements OnInit {
  title = "ATLAS";
  cameras: Camera[] = [];

  constructor(private cameraService: CameraService) { }

  ngOnInit(): void {
    this.cameraService.getCameras()
      .then(cameras => this.cameras = cameras);
  }
}
