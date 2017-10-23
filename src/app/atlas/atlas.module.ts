import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { SharedModule } from '../shared/shared.module';
import { CameraModule } from '../camera/camera.module';

import { AtlasBatteriesComponent } from './atlas-batteries.component';
import { AtlasCamerasComponent } from './atlas-cameras.component';
import { AtlasDashboardComponent } from './atlas-dashboard.component';
import { AtlasEfoysComponent } from './atlas-efoys.component';
import { AtlasRoutingModule } from './atlas-routing.module';
import { AtlasService } from './atlas.service';
import { AtlasStatusComponent } from './atlas-status.component';
import { AtlasTimeseriesComponent } from './atlas-timeseries.component';

@NgModule({
  imports: [
    AtlasRoutingModule,
    CameraModule,
    CommonModule,
    SharedModule,
  ],
  exports: [],
  declarations: [
    AtlasBatteriesComponent,
    AtlasCamerasComponent,
    AtlasDashboardComponent,
    AtlasEfoysComponent,
    AtlasStatusComponent,
    AtlasTimeseriesComponent,
  ],
  providers: [AtlasService],
})
export class AtlasModule { }
