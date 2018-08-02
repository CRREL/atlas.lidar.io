import { Component, OnInit } from '@angular/core';

import { AtlasStatus } from './atlas-status';
import { AtlasService } from './atlas.service';
import { Camera } from '../camera/camera';
import { CameraService } from '../camera/camera.service';

@Component({
    selector: 'app-atlas-dashboard',
    templateUrl: './atlas-dashboard.component.html',
    styleUrls: ['./atlas-dashboard.component.css'],
})
export class AtlasDashboardComponent implements OnInit {
    atlasStatus: AtlasStatus;
    cameras: Camera[];
    cameraNames = ['ATLAS_CAM', 'ATLAS_CAM2', 'HEL_ATLASNORTH', 'HEL_DUAL_2', 'HEL_DUAL_1', 'HEL_BERGCAM3_1', 'HEL_BERGCAM3_2'];

    constructor(private atlasService: AtlasService, private cameraService: CameraService) { }

    ngOnInit(): void {
        this.atlasService.getAtlasStatus()
            .then(atlasStatus => this.atlasStatus = atlasStatus);
        this.cameraService.getCameraDetails(this.cameraNames)
            .then(cameras => this.cameras = cameras);
    }
}
