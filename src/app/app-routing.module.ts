import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';

import { HelBergcamComponent } from './hel-bergcam.component';

const routes: Routes = [
  { path: '', redirectTo: 'dashboard', pathMatch: 'full' },
  { path: 'hel-bergcam', component: HelBergcamComponent },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule],
})
export class AppRoutingModule { }
