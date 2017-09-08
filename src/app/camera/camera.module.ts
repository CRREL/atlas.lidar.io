import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { CameraLatestImageComponent } from './camera-latest-image.component';
import { CameraService } from './camera.service';

@NgModule({
  imports: [
    CommonModule
  ],
  exports: [CameraLatestImageComponent],
  declarations: [CameraLatestImageComponent],
  providers: [CameraService],
})
export class CameraModule { }
