import { Component, Input } from '@angular/core';
import { AtlasStatus } from './atlas-status';
import { AtlasEfoyStatus } from './atlas-efoy-status';
import { Camera } from '../camera/camera';

@Component({
  selector: 'app-atlas-status',
  templateUrl: './atlas-status.component.html',
})
export class AtlasStatusComponent {
  @Input() status: AtlasStatus;
  @Input() cameras: Camera[];

  getSystemStatusItems(): StatusItem[] {
    return [
      this.getHeartbeatItem(),
      this.getRieglSwitchItem(),
      this.getScannerItem(),
    ];
  }

  getPowerStatusItems(): StatusItem[] {
    return this.status.efoys.map(efoy => this.getEfoyStatusItem(efoy));
  }

  getCameraStatusItems(): StatusItem[] {
    return this.cameras.map(camera => this.getCameraStatusItem(camera));
  }

  getHeartbeatItem(): StatusItem {
    const delta = (new Date()).getTime() - this.status.last_heartbeat_received.getTime();
    let value, style;
    if (delta < hoursToMilliseconds(1.5)) {
      value = 'on time';
      style = 'success';
    } else if (delta < hoursToMilliseconds(6.5)) {
      value = 'late';
      style = 'warning';
    } else {
      value = 'stopped';
      style = 'danger';
    }

    return {
      key: 'Heartbeats',
      value: value,
      style: style,
    };
  }

  getRieglSwitchItem(): StatusItem {
    return {
      key: 'Scanner and housing power',
      value: this.status.is_riegl_switch_on ? 'on' : 'off',
      style: this.status.is_riegl_switch_on ? 'success' : 'warning'
    };
  }

  getScannerItem(): StatusItem {
    const delta = (new Date()).getTime() - this.status.last_scan.start.getTime();
    let value, style;
    if (delta < hoursToMilliseconds(26)) {
      if (this.status.last_scan.end) {
        value = 'on time';
        style = 'success';
      } else {
        value = 'aborted';
        style = 'dark';
      }
    } else if (delta < hoursToMilliseconds(48)) {
      value = 'late';
      style = 'warning';
    } else {
      value = 'stopped';
      style = 'danger';
    }
    return {
      key: 'Last scan',
      value: value,
      style: style,
    };
  }

  getEfoyStatusItem(efoy: AtlasEfoyStatus): StatusItem {
    const value = efoy.state;
    let style;
    if (value === 'auto on') {
      style = 'info';
    } else if (value === 'auto off') {
      style = 'success';
    } else if (value === 'freeze protection') {
      style = 'warning';
    }
    return {
      key: 'EFOY ' + efoy.id,
      value: value,
      style: style,
    };
  }

  getCameraStatusItem(camera: Camera): StatusItem {
    const delta = (new Date()).getTime() - camera.latest_image.datetime.getTime();
    let value, style;
    if (delta < hoursToMilliseconds(24)) {
      value = 'on time';
      style = 'success';
    } else if (delta < hoursToMilliseconds(48)) {
      value = 'late';
      style = 'warning';
    } else {
      value = 'stopped';
      style = 'danger';
    }
    return {
      key: camera.name,
      value: value,
      style: style,
    };
  }
}

class StatusItem {
  key: string;
  value: string;
  style: string;
}

function hoursToMilliseconds(hours: number): number {
  return hours * 60 * 60 * 1000;
}
