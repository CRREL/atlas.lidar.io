import { Injectable } from "@angular/core";
import { Http } from "@angular/http";
import { Camera } from "./camera";
import { environment } from "../environments/environment";
import "rxjs/add/operator/toPromise";

@Injectable()
export class CameraService {

  private camerasUrl = environment.glacioApiServer + "/cameras";

  constructor(private http: Http) { }

  getCameras(): Promise<Camera[]> {
    return this.http.get(this.camerasUrl)
      .toPromise()
      .then(response => response.json() as Camera[])
      .catch(this.handleError);
  }

  private handleError(error: any): Promise<any> {
    console.error("An error occurred", error);
    return Promise.reject(error.message || error);
  }
}
