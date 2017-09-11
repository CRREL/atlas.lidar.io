import { Component, Input } from '@angular/core';
import { AtlasBatteryStatus } from './atlas-battery-status';

@Component({
  selector: 'app-atlas-batteries',
  templateUrl: './atlas-batteries.component.html',
})
export class AtlasBatteriesComponent {
  @Input() batteries: AtlasBatteryStatus[];
}
