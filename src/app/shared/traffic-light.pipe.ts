import { Pipe, PipeTransform } from '@angular/core';

@Pipe({ name: "trafficLight" })
export class TrafficLightPipe implements PipeTransform {
  transform(n: number): string {
    if (n > 70) {
      return 'bg-success';
    } else if (n > 40) {
      return 'bg-warning';
    } else {
      return 'bg-danger';
    }
  }
}
