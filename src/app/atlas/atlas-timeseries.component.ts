import { OnInit, Input, Component, ViewChild, ViewEncapsulation, ElementRef } from '@angular/core';

import * as d3 from 'd3';
import * as d3_scale_chromatic from 'd3-scale-chromatic';

import { AtlasTimeseries } from './atlas-timeseries';

@Component({
  selector: 'app-atlas-timeseries',
  templateUrl: './atlas-timeseries.component.html',
  styleUrls: ['./atlas-timeseries.component.css'],
  encapsulation: ViewEncapsulation.None,
})
export class AtlasTimeseriesComponent implements OnInit {
  @ViewChild('atlasTimeseriesChart') private chartContainer: ElementRef;
  @Input() timeseries: AtlasTimeseries;
  private margin: any = { top: 20, bottom: 40, left: 30, right: 20 };

  ngOnInit(): void {
    this.createChart();
  }

  createChart(): void {
    const stateOfCharge = Object.keys(this.timeseries.states_of_charge).map(id => {
      return {
        name: 'Battery ' + id,
        values: this.timeseries.states_of_charge[id].map((d, i) => {
          return {
            datetime: this.timeseries.datetimes[i],
            value: d,
          };
        })
      };
    });
    const fuelPercentage = Object.keys(this.timeseries.efoy_fuel_percentage).map(id => {
      return {
        name: 'EFOY ' + id,
        values: this.timeseries.efoy_fuel_percentage[id].map((d, i) => {
          return {
            datetime: this.timeseries.datetimes[i],
            value: d,
          };
        })
      };
    });

    const element = this.chartContainer.nativeElement;
    const width = element.offsetWidth - this.margin.left - this.margin.right;
    const height = element.offsetHeight - this.margin.top - this.margin.bottom;

    const svg = d3.select(element).append('svg')
      .attr('width', element.offsetWidth)
      .attr('height', element.offsetHeight);

    const g = svg.append('g').attr('transform', `translate(${this.margin.left},${this.margin.top})`);
    const xScale = d3.scaleTime().domain(d3.extent(this.timeseries.datetimes)).range([0, width]);
    const yScale = d3.scaleLinear().domain([0, 100]).range([height, 0]);
    const batteryColourScale = d3.scaleOrdinal(d3_scale_chromatic.schemePaired).domain(stateOfCharge.map(d => d.name));
    const efoyColourScale = d3.scaleOrdinal(d3_scale_chromatic.schemePastel1).domain(fuelPercentage.map(d => d.name));

    const line = d3.line<any>()
      .curve(d3.curveBasis)
      .x(d => xScale(d.datetime))
      .y(d => yScale(d.value));

    g.selectAll('.fuel-percentage')
      .data(fuelPercentage)
      .enter()
      .append('g')
      .attr('class', 'fuel-percentage')
      .append('path')
      .attr('class', 'line')
      .attr('d', d => line(d.values))
      .style('stroke', d => efoyColourScale(d.name));

    g.selectAll('.state-of-charge')
      .data(stateOfCharge)
      .enter()
      .append('g')
      .attr('class', 'state-of-charge')
      .append('path')
      .attr('class', 'line')
      .attr('d', d => line(d.values))
      .style('stroke', d => batteryColourScale(d.name));

    g.append('g')
      .attr('transform', `translate(0,${height})`)
      .call(d3.axisBottom(xScale));

    g.append('g')
      .call(d3.axisLeft(yScale));
  }
}
