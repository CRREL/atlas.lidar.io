import "rxjs/add/operator/switchMap";
import { Component, OnInit } from "@angular/core";
import { ActivatedRoute, ParamMap } from "@angular/router";

import { Camera } from "./camera";
import { CameraService } from "./camera.service";

@Component({
  selector: "camera-detail",
  templateUrl: "./camera-detail.component.html",
  styleUrls: ["./camera-detail.component.css"],
})
export class CameraDetailComponent implements OnInit {
  camera: Camera;

  constructor(private cameraService: CameraService, private route: ActivatedRoute) { }

  ngOnInit(): void {
    this.route.paramMap
      .switchMap((params: ParamMap) => this.cameraService.getCamera(params.get("name")))
      .subscribe(camera => this.camera = camera);
  }
}
