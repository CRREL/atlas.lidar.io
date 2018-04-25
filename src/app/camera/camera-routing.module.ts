import { NgModule } from '@angular/core';
import { RouterModule } from '@angular/router';
import { ColumbiaComponent } from './columbia.component';

@NgModule({
  imports: [RouterModule.forChild([
    { path: 'columbia', component: ColumbiaComponent }
  ])],
  exports: [RouterModule]
})
export class CameraRoutingModule { }
