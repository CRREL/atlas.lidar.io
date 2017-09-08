import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { CameraFigureComponent } from './camera-figure.component';
import { CameraService } from './camera.service';

@NgModule({
  imports: [
    CommonModule
  ],
  exports: [CameraFigureComponent],
  declarations: [CameraFigureComponent],
  providers: [CameraService],
})
export class CameraModule { }
