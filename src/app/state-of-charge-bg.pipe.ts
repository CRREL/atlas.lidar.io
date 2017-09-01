import { Pipe, PipeTransform } from "@angular/core";

@Pipe({ name: "stateOfChargeBg" })
export class StateOfChargeBgPipe implements PipeTransform {
  transform(stateOfCharge: number): string {
    if (stateOfCharge > 70) {
      return "bg-success";
    } else if (stateOfCharge > 40) {
      return "bg-warning";
    } else {
      return "bg-danger";
    }
  }
}
