import { Component, Input } from '@angular/core';
import { AtlasEfoyStatus } from './atlas-efoy-status';

@Component({
  selector: 'atlas-efoys',
  templateUrl: './atlas-efoys.component.html',
})
export class AtlasEfoysComponent {
  @Input() efoys: AtlasEfoyStatus[];
}
