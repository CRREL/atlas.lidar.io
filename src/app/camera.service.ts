import { Injectable } from "@angular/core";
import { Http } from "@angular/http";
import { Camera } from "./camera";

@Injectable()
export class CameraService {

  private camerasUrl = "cameras";

  constructor(private http: Http) { }

  getCameras(): Promise<Camera[]> {
    return this.http.get(this.camerasUrl)
      .toPromise()
      .then(response => response.json().data as Camera[])
      .catch(this.handleError);
  }
}
