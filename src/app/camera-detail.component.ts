import { Component, Input } from "@angular/core";

import { Camera } from "./camera";

@Component({
  selector: "camera-detail",
  templateUrl: "./camera-detail.component.html",
  styleUrls: ["./camera-detail.component.css"],
})
export class CameraDetailComponent {
  @Input() camera: Camera;
}
