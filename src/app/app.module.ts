import { BrowserModule } from "@angular/platform-browser";
import { NgModule } from "@angular/core";
import { HttpModule } from "@angular/http";

import { AppComponent } from "./app.component";
import { CameraDetailComponent } from "./camera-detail.component";
import { AppRoutingModule } from "./app-routing.module";
import { DashboardComponent } from "./dashboard.component";
import { CameraService } from "./camera.service";

@NgModule({
  declarations: [
    AppComponent,
    CameraDetailComponent,
    DashboardComponent,
  ],
  imports: [
    AppRoutingModule,
    BrowserModule,
    HttpModule,
  ],
  providers: [HttpModule, CameraService],
  bootstrap: [AppComponent]
})
export class AppModule { }
