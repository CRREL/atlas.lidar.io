import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { HttpModule } from '@angular/http';

import { AppComponent } from './app.component';
import { AppRoutingModule } from './app-routing.module';

import { AtlasModule } from './atlas/atlas.module';
import { CameraModule } from './camera/camera.module';
import { HelBergcamModule } from './hel-bergcam/hel-bergcam.module';

@NgModule({
  declarations: [AppComponent],
  imports: [
    AppRoutingModule,
    AtlasModule,
    BrowserModule,
    CameraModule,
    HelBergcamModule,
    HttpModule,
  ],
  providers: [HttpModule],
  bootstrap: [AppComponent]
})
export class AppModule { }
