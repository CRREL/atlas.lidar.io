import { AtlasBattery } from "./atlas-battery";

export interface AtlasStatus {
  last_heartbeat_received: Date;
  batteries: AtlasBattery[];
}
