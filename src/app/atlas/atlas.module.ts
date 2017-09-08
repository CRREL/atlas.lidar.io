import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { SharedModule } from '../shared/shared.module';
import { CameraModule } from '../camera/camera.module';

import { AtlasService } from './atlas.service';
import { AtlasRoutingModule } from './atlas-routing.module';

import { AtlasDashboardComponent } from './atlas-dashboard.component';
import { AtlasBatteriesComponent } from './atlas-batteries.component';
import { AtlasEfoysComponent } from './atlas-efoys.component';
import { AtlasTimeseriesComponent } from './atlas-timeseries.component';

@NgModule({
  imports: [
    CommonModule, AtlasRoutingModule, SharedModule, CameraModule
  ],
  exports: [],
  declarations: [AtlasDashboardComponent, AtlasBatteriesComponent, AtlasEfoysComponent, AtlasTimeseriesComponent],
  providers: [AtlasService],
})
export class AtlasModule { }
