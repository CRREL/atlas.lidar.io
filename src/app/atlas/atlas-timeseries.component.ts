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
  private margin: any = { top: 20, bottom: 40, left: 70, right: 40 };
  private mainChartPercentage = 0.6;
  private padding = 40;

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
    const rieglSwitch = this.timeseries.is_riegl_switch_on.map((d, i) => {
      return {
        datetime: this.timeseries.datetimes[i],
        value: d,
      };
    })
    const efoyState = Object.keys(this.timeseries.efoy_state).map(id => {
      return {
        name: efoyName(id),
        values: this.timeseries.efoy_state[id].map((d, i) => {
          return {
            datetime: this.timeseries.datetimes[i],
            value: d,
            id: +id,
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
    const yScale = d3.scaleLinear().domain([0, 100]).range([height * this.mainChartPercentage, 0]);
    const yScale2 = d3.scaleBand().domain([efoyName(2), efoyName(1), 'Riegl switch'])
      .range([height, height * this.mainChartPercentage + this.padding])
      .padding(0.2);
    const batteryColourScale = d3.scaleOrdinal(d3_scale_chromatic.schemePaired).domain(stateOfCharge.map(d => d.name));
    const efoyColourScale = d3.scaleOrdinal(d3_scale_chromatic.schemePastel1).domain(fuelPercentage.map(d => d.name));

    const line = d3.line<any>()
      .curve(d3.curveBasis)
      .x(d => xScale(d.datetime))
      .y(d => yScale(d.value));
    const switchArea = d3.area<any>()
      .defined(d => d.value)
      .x(d => xScale(d.datetime))
      .y0(d => yScale2('Riegl switch'))
      .y1(d => yScale2('Riegl switch') + yScale2.bandwidth());
    const efoyArea = d3.area<any>()
      .defined(d => d.value == 'auto on')
      .x(d => xScale(d.datetime))
      .y0(d => yScale2(efoyName(d.id)))
      .y1(d => yScale2(efoyName(d.id)) + yScale2.bandwidth());

    g.append('g')
      .datum(rieglSwitch)
      .attr('class', 'riegl-switch')
      .append('path')
      .attr('class', 'area')
      .attr('d', switchArea)
      .style('fill', d => 'green')
      .style('opacity', 0.2);

    g.selectAll('.efoy-state')
      .data(efoyState)
      .enter()
      .append('g')
      .attr('class', 'efoy-state')
      .append('path')
      .attr('class', 'area')
      .attr('d', d => efoyArea(d.values))
      .style('fill', d => efoyColourScale(d.name))
      .style('opacity', 0.5);

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
      .attr('transform', `translate(0,${height * this.mainChartPercentage})`)
      .call(d3.axisBottom(xScale));

    g.append('g')
      .call(d3.axisLeft(yScale));

    g.append('g')
      .call(d3.axisLeft(yScale2));
    g.append('g')
      .attr('transform', `translate(0,${height})`)
      .call(d3.axisBottom(xScale).ticks(0));
  }
}

function efoyName(id: any): string {
  return "EFOY " + id;
}
