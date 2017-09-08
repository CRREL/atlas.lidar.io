import { NgModule } from '@angular/core';
import { RouterModule } from '@angular/router';

import { AtlasDashboardComponent } from './atlas-dashboard.component';

@NgModule({
  imports: [RouterModule.forChild([
    { path: 'dashboard', component: AtlasDashboardComponent }
  ])],
  exports: [RouterModule]
})
export class AtlasRoutingModule { }
