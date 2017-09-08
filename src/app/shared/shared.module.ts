import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { TrafficLightPipe } from './traffic-light.pipe';

@NgModule({
  imports: [
    CommonModule
  ],
  declarations: [TrafficLightPipe],
  exports: [CommonModule, TrafficLightPipe],
})
export class SharedModule { }
