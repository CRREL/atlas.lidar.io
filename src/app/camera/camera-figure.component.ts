import { Component, OnInit, Input } from '@angular/core';
import { CameraService } from './camera.service';
import { Camera } from './camera';

@Component({
  selector: 'camera-figure',
  templateUrl: './camera-figure.component.html',
})
export class CameraFigureComponent implements OnInit {
  @Input() camera: Camera;
  @Input() name: string;

  constructor(private cameraService: CameraService) { }

  ngOnInit(): void {
    if (!this.camera) {
      this.cameraService.getCamera(this.name).then(camera => this.camera = camera);
    }
  }
}
