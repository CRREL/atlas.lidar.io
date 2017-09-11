import { NgModule } from '@angular/core';
import { RouterModule } from '@angular/router';
import { HelBergcamComponent } from './hel-bergcam.component';

@NgModule({
  imports: [RouterModule.forChild([
    { path: 'hel-bergcam', component: HelBergcamComponent }
  ])],
  exports: [RouterModule]
})
export class HelBergcamRoutingModule { }
