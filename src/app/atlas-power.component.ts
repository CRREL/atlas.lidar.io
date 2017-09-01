import { Component, OnInit, ViewChild, ViewEncapsulation, ElementRef } from "@angular/core";
import { AtlasService } from "./atlas.service";
import { AtlasPowerHistory } from "./atlas-power-history";
import * as d3 from "d3";

@Component({
  selector: "atlas-power",
  templateUrl: "./atlas-power.component.html",
  styleUrls: ["./atlas-power.component.css"],
  encapsulation: ViewEncapsulation.None,
})
export class AtlasPowerComponent implements OnInit {
  @ViewChild('atlasPowerChart') private chartContainer: ElementRef;
  history: AtlasPowerHistory;
  private margin: any = { top: 20, bottom: 20, left: 40, right: 20 };

  constructor(private atlasService: AtlasService) { }

  ngOnInit(): void {
    this.atlasService.getAtlasPowerHistory().then(atlasPowerHistory => {
      this.history = atlasPowerHistory;
      this.createChart();
    });
  }

  createChart(): void {
    const batteries = [{
      name: "Battery 1",
      values: this.datify(this.history.state_of_charge_1),
    },
    {
      name: "Battery 2",
      values: this.datify(this.history.state_of_charge_2),
    }
    ];
    const element = this.chartContainer.nativeElement;
    const width = element.offsetWidth - this.margin.left - this.margin.right;
    const height = element.offsetHeight - this.margin.top - this.margin.bottom;

    const svg = d3.select(element).append("svg")
      .attr("width", element.offsetWidth)
      .attr("height", element.offsetHeight)

    const g = svg.append("g").attr("transform", `translate(${this.margin.left},${this.margin.top})`);
    const xScale = d3.scaleTime().domain(d3.extent(this.history.datetime)).range([0, width]);
    const yScale = d3.scaleLinear().domain([0, 100]).rangeRound([height, 0]);
    const zScale = d3.scaleOrdinal(d3.schemeCategory10).domain(batteries.map(b => b.name));
    const line = d3.line<PowerRecord>()
      .curve(d3.curveBasis)
      .x(d => xScale(d.datetime))
      .y(d => yScale(d.state_of_charge));

    g.append("g")
      .attr("class", "axis axis-x")
      .attr("transform", `translate(0,${height})`)
      .call(d3.axisBottom(xScale));

    g.append("g")
      .attr("class", "axis axis-y")
      .call(d3.axisLeft(yScale));

    var battery = g.selectAll(".battery")
      .data(batteries)
      .enter().append("g")
      .attr("class", "battery");

    battery.append("path")
      .attr("class", "line")
      .attr("d", d => line(d.values))
      .style("stroke", d => zScale(d.name));
  }

  private datify(state_of_charge: number[]): PowerRecord[] {
    return this.history.datetime.map((datetime, i) => {
      var powerRecord = { datetime: datetime, state_of_charge: state_of_charge[i] };
      return powerRecord;
    });
  }
}

interface PowerRecord {
  datetime: Date;
  state_of_charge: number;
}
