import { AtlasCartridgeStatus } from './atlas-cartridge-status';

export interface AtlasEfoyStatus {
  id: number;
  state: string;
  active_cartridge: string;
  active_cartridge_consumed: number;
  voltage: number;
  current: number;
  cartridges: AtlasCartridgeStatus[];
}
