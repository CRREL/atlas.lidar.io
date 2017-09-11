import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { HelBergcamRoutingModule } from './hel-bergcam-routing.module';
import { HelBergcamComponent } from './hel-bergcam.component';
import { CameraModule } from '../camera/camera.module';

@NgModule({
  imports: [
    CommonModule, HelBergcamRoutingModule, CameraModule,
  ],
  declarations: [HelBergcamComponent],
})
export class HelBergcamModule { }
