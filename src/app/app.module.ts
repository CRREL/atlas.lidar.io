import { BrowserModule } from "@angular/platform-browser";
import { NgModule } from "@angular/core";
import { HttpModule } from "@angular/http";

import { AppComponent } from "./app.component";
import { AppRoutingModule } from "./app-routing.module";
import { DashboardComponent } from "./dashboard.component";
import { StateOfChargeBgPipe } from "./state-of-charge-bg.pipe";
import { AtlasService } from "./atlas.service";

@NgModule({
  declarations: [
    AppComponent,
    DashboardComponent,
    StateOfChargeBgPipe,
  ],
  imports: [
    AppRoutingModule,
    BrowserModule,
    HttpModule,
  ],
  providers: [HttpModule, AtlasService],
  bootstrap: [AppComponent]
})
export class AppModule { }
