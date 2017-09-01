import { Component, OnInit } from "@angular/core";

import { AtlasStatus } from "./atlas-status";
import { AtlasService } from "./atlas.service";

@Component({
  selector: "dashboard",
  templateUrl: "./dashboard.component.html",
  styleUrls: ["./dashboard.component.css"],
})
export class DashboardComponent implements OnInit {
  atlasStatus: AtlasStatus = null;

  constructor(private atlasService: AtlasService) { }

  ngOnInit(): void {
    this.atlasService.getAtlasStatus()
      .then(atlasStatus => this.atlasStatus = atlasStatus);
  }
}
