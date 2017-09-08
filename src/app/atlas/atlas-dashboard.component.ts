import { Component, OnInit } from '@angular/core';

import { AtlasStatus } from './atlas-status';
import { AtlasService } from './atlas.service';

@Component({
  selector: 'atlas-dashboard',
  templateUrl: './atlas-dashboard.component.html',
  styleUrls: ['./atlas-dashboard.component.css'],
})
export class AtlasDashboardComponent implements OnInit {
  atlasStatus: AtlasStatus = null;

  constructor(private atlasService: AtlasService) { }

  ngOnInit(): void {
    this.atlasService.getAtlasStatus()
      .then(atlasStatus => this.atlasStatus = atlasStatus);
  }
}
