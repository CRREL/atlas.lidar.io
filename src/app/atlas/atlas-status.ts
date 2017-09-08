import { AtlasBatteryStatus } from './atlas-battery-status';
import { AtlasEfoyStatus } from './atlas-efoy-status';
import { AtlasTimeseries } from './atlas-timeseries';

export interface AtlasStatus {
  last_heartbeat_received: Date;
  batteries: AtlasBatteryStatus[];
  efoys: AtlasEfoyStatus[];
  timeseries: AtlasTimeseries;
}
