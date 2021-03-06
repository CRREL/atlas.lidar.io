import { Component, Input } from '@angular/core';
import { Camera } from '../camera/camera';

@Component({
  selector: 'app-atlas-cameras',
  templateUrl: './atlas-cameras.component.html',
})
export class AtlasCamerasComponent {
  @Input() cameras: Camera[];
}
