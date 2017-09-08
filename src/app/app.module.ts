import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { HttpModule } from '@angular/http';

import { AppComponent } from './app.component';
import { AppRoutingModule } from './app-routing.module';
import { HelBergcamComponent } from './hel-bergcam.component';

import { AtlasModule } from './atlas/atlas.module';
import { CameraModule } from './camera/camera.module';

@NgModule({
  declarations: [
    AppComponent,
    HelBergcamComponent,
  ],
  imports: [
    AppRoutingModule,
    AtlasModule,
    CameraModule,
    BrowserModule,
    HttpModule,
  ],
  providers: [HttpModule],
  bootstrap: [AppComponent]
})
export class AppModule { }
