import { Component, OnInit, ViewChild, ViewEncapsulation, ElementRef } from "@angular/core";
import { AtlasService } from "./atlas.service";
import { AtlasPowerHistory } from "./atlas-power-history";
import * as d3 from "d3";
import * as d3_scale_chromatic from "d3-scale-chromatic";

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
      values: this.datifyPowerRecord(this.history.state_of_charge_1),
    },
    {
      name: "Battery 2",
      values: this.datifyPowerRecord(this.history.state_of_charge_2),
    }
    ];
    const efoys = [{
      name: "EFOY 1",
      values: this.datifyEfoyRecord(this.history.efoy_1_current),
    },
    {
      name: "EFOY 2",
      values: this.datifyEfoyRecord(this.history.efoy_2_current),
    }];
    const element = this.chartContainer.nativeElement;
    const width = element.offsetWidth - this.margin.left - this.margin.right;
    const height = element.offsetHeight - this.margin.top - this.margin.bottom;

    const svg = d3.select(element).append("svg")
      .attr("width", element.offsetWidth)
      .attr("height", element.offsetHeight)

    const g = svg.append("g").attr("transform", `translate(${this.margin.left},${this.margin.top})`);
    const xScale = d3.scaleTime().domain(d3.extent(this.history.datetime)).range([0, width]);
    const yScaleBattery = d3.scaleLinear().domain([0, 100]).rangeRound([height, 0]);
    const yScaleEfoy = d3.scaleLinear().domain([0, 4]).rangeRound([height, 0]);
    const zScaleBatteries = d3.scaleOrdinal(d3_scale_chromatic.schemePaired).domain(batteries.map(b => b.name));
    const zScaleEfoys = d3.scaleOrdinal(d3_scale_chromatic.schemePastel1).domain(efoys.map(b => b.name));
    const line = d3.line<PowerRecord>()
      .curve(d3.curveBasis)
      .x(d => xScale(d.datetime))
      .y(d => yScaleBattery(d.state_of_charge));
    const area = d3.area<EfoyRecord>()
      .curve(d3.curveBasis)
      .x(d => xScale(d.datetime))
      .y0(yScaleEfoy(0))
      .y1(d => yScaleEfoy(d.current));

    var efoy = g.selectAll(".efoy")
      .data(efoys)
      .enter().append("g")
      .attr("class", "efoy");

    efoy.append("path")
      .attr("class", "area")
      .attr("d", d => area(d.values))
      .style("opacity", 0.6)
      .style("fill", d => zScaleEfoys(d.name));

    var battery = g.selectAll(".battery")
      .data(batteries)
      .enter().append("g")
      .attr("class", "battery");

    battery.append("path")
      .attr("class", "line")
      .attr("d", d => line(d.values))
      .style("stroke", d => zScaleBatteries(d.name));

    g.append("g")
      .attr("class", "axis axis-x")
      .attr("transform", `translate(0,${height})`)
      .call(d3.axisBottom(xScale));

    g.append("g")
      .attr("class", "axis axis-y")
      .call(d3.axisLeft(yScaleBattery));
  }

  private datifyPowerRecord(state_of_charge: number[]): PowerRecord[] {
    return this.history.datetime.map((datetime, i) => {
      var powerRecord = { datetime: datetime, state_of_charge: state_of_charge[i] };
      return powerRecord;
    });
  }

  private datifyEfoyRecord(current: number[]): EfoyRecord[] {
    return this.history.datetime.map((datetime, i) => {
      var efoyRecord = { datetime: datetime, current: current[i] };
      return efoyRecord;
    });
  }
}

interface PowerRecord {
  datetime: Date;
  state_of_charge: number;
}

interface EfoyRecord {
  datetime: Date;
  current: number;
}
