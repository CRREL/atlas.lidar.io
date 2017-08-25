import { Component, OnInit } from "@angular/core";

import { AtlasStatus } from "./atlas-status";
import { AtlasService } from "./atlas.service";
import { Camera } from "./camera";
import { CameraService } from "./camera.service";

@Component({
  selector: "dashboard",
  templateUrl: "./dashboard.component.html",
  styleUrls: ["./dashboard.component.css"],
})
export class DashboardComponent implements OnInit {
  atlasStatus: AtlasStatus = null;
  cameras: Camera[] = [];

  constructor(private cameraService: CameraService, private atlasService: AtlasService) { }

  ngOnInit(): void {
    this.cameraService.getCameras()
      .then(cameras => this.cameras = cameras as Camera[]);
    this.atlasService.getAtlasStatus()
      .then(atlasStatus => this.atlasStatus = atlasStatus);
  }
}
