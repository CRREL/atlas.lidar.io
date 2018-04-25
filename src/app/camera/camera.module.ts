import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { CameraFigureComponent } from './camera-figure.component';
import { CameraRoutingModule } from './camera-routing.module';
import { CameraService } from './camera.service';
import { ColumbiaComponent } from './columbia.component';

@NgModule({
  imports: [
    CommonModule, CameraRoutingModule
  ],
  exports: [CameraFigureComponent],
  declarations: [CameraFigureComponent, ColumbiaComponent],
  providers: [CameraService],
})
export class CameraModule { }
