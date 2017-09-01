import { BrowserModule } from "@angular/platform-browser";
import { NgModule } from "@angular/core";
import { HttpModule } from "@angular/http";

import { AppComponent } from "./app.component";
import { AppRoutingModule } from "./app-routing.module";
import { DashboardComponent } from "./dashboard.component";
import { StateOfChargeBgPipe } from "./state-of-charge-bg.pipe";
import { AtlasService } from "./atlas.service";
import { AtlasPowerComponent } from "./atlas-power.component";
import { CameraLatestImageComponent } from "./camera-latest-image.component";
import { CameraService } from "./camera.service";

@NgModule({
  declarations: [
    AppComponent,
    AtlasPowerComponent,
    DashboardComponent,
    StateOfChargeBgPipe,
    CameraLatestImageComponent,
  ],
  imports: [
    AppRoutingModule,
    BrowserModule,
    HttpModule,
  ],
  providers: [HttpModule, AtlasService, CameraService],
  bootstrap: [AppComponent]
})
export class AppModule { }
