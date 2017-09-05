import { AtlasBattery } from "./atlas-battery";
import { AtlasEfoy } from "./atlas-efoy";

export interface AtlasStatus {
  last_heartbeat_received: Date;
  batteries: AtlasBattery[];
  efoys: AtlasEfoy[];
}
