import { Component } from '@angular/core';

import { CameraService } from './camera.service';
import { Image } from './camera';

@Component({
  selector: 'app-columbia',
  templateUrl: './columbia.component.html',
})
export class ColumbiaComponent {
  images: Image[];
  private CAMERA_NAME = 'COLUMBIA';

  constructor(private cameraService: CameraService) { }

  ngOnInit(): void {
    this.cameraService.getImages(this.CAMERA_NAME).then(images => this.images = images);
  }
}
