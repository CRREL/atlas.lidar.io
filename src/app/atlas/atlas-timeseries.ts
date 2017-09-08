export interface AtlasTimeseries {
  datetimes: Date[];
  states_of_charge: { id: number[] }[];
  efoy_current: { id: number[] }[];
  efoy_voltage: { id: number[] }[];
  efoy_fuel_percentage: { id: number[] }[];
}
