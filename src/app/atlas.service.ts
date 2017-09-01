import { Injectable } from "@angular/core";
import { Http } from "@angular/http";
import { environment } from "../environments/environment";
import "rxjs/add/operator/toPromise";

import { AtlasStatus } from "./atlas-status";
import { AtlasPowerHistory } from "./atlas-power-history";

@Injectable()
export class AtlasService {
  private atlasStatusUrl = environment.glacioApiServer + "/atlas/status";
  private atlasPowerHistoryUrl = environment.glacioApiServer + "/atlas/power/history";

  constructor(private http: Http) { }

  getAtlasStatus(): Promise<AtlasStatus> {
    return this.http.get(this.atlasStatusUrl)
      .toPromise()
      .then(response => response.json() as AtlasStatus)
      .catch(this.handleError);
  }

  getAtlasPowerHistory(): Promise<AtlasPowerHistory> {
    return this.http.get(this.atlasPowerHistoryUrl)
      .toPromise()
      .then(response => {
        var history = response.json() as AtlasPowerHistory;
        history.datetime = history.datetime.map(s => new Date(s));
        return history;
      })
      .catch(this.handleError);
  }

  private handleError(error: any): Promise<any> {
    console.error("An error occured", error);
    return Promise.reject(error.message || error);
  }
}
