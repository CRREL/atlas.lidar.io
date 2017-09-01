import { BrowserModule } from "@angular/platform-browser";
import { NgModule } from "@angular/core";
import { HttpModule } from "@angular/http";

import { AppComponent } from "./app.component";
import { AppRoutingModule } from "./app-routing.module";
import { DashboardComponent } from "./dashboard.component";
import { StateOfChargeBgPipe } from "./state-of-charge-bg.pipe";
import { AtlasService } from "./atlas.service";
import { AtlasPowerComponent } from "./atlas-power.component";

@NgModule({
  declarations: [
    AppComponent,
    AtlasPowerComponent,
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
