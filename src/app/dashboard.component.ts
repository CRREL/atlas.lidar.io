import { Component, OnInit } from "@angular/core";

import { Camera } from "./camera";
import { CameraService } from "./camera.service";

@Component({
  selector: "dashboard",
  templateUrl: "./dashboard.component.html",
  styleUrls: ["./dashboard.component.css"],
})
export class DashboardComponent implements OnInit {
  cameras: Camera[] = [];

  constructor(private cameraService: CameraService) { }

  ngOnInit(): void {
    this.cameraService.getCameras()
      .then(cameras => this.cameras = cameras as Camera[]);
  }
}
