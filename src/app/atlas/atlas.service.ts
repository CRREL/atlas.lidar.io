import { Injectable } from '@angular/core';
import { Http } from '@angular/http';
import { environment } from '../../environments/environment';
import 'rxjs/add/operator/toPromise';

import { AtlasStatus } from './atlas-status';

@Injectable()
export class AtlasService {
  private atlasStatusUrl = environment.glacioApiServer + '/atlas/status';

  constructor(private http: Http) { }

  getAtlasStatus(): Promise<AtlasStatus> {
    return this.http.get(this.atlasStatusUrl)
      .toPromise()
      .then(response => {
        const status = response.json() as AtlasStatus;
        status.last_heartbeat_received = new Date(status.last_heartbeat_received);
        status.timeseries.datetimes = status.timeseries.datetimes.map(s => new Date(s));
        return status;
      })
      .catch(this.handleError);
  }

  private handleError(error: any): Promise<any> {
    console.error('An error occured', error);
    return Promise.reject(error.message || error);
  }
}
