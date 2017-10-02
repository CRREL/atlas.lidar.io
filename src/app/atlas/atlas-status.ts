import { AtlasBatteryStatus } from './atlas-battery-status';
import { AtlasEfoyStatus } from './atlas-efoy-status';
import { AtlasTimeseries } from './atlas-timeseries';
import { AtlasLastScan } from './atlas-last-scan';

export interface AtlasStatus {
  last_heartbeat_received: Date;
  last_scan: AtlasLastScan;
  batteries: AtlasBatteryStatus[];
  efoys: AtlasEfoyStatus[];
  timeseries: AtlasTimeseries;
  is_riegl_switch_on: boolean;
}
