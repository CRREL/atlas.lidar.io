import { Injectable } from '@angular/core';
import { Http } from '@angular/http';
import { Camera, Image } from './camera';
import { environment } from '../../environments/environment';
import 'rxjs/add/operator/toPromise';

@Injectable()
export class CameraService {

  private camerasUrl = environment.glacioApiServer + '/cameras';

  constructor(private http: Http) { }

  getCameras(): Promise<Camera[]> {
    return this.http.get(this.camerasUrl)
      .toPromise()
      .then(response => response.json() as Camera[])
      .catch(this.handleError);
  }

  getCamera(name: string): Promise<Camera> {
    return this.http.get(this.camerasUrl + '/' + name)
      .toPromise()
      .then(response => jsonToCameraDetail(response.json()))
      .catch(this.handleError);
  }

  getCameraDetails(names: string[]): Promise<Camera[]> {
    return Promise.all(names.map(name =>
      this.http.get(this.getCameraUrl(name))
        .toPromise()
        .then(response => jsonToCameraDetail(response.json()))
        .catch(this.handleError)));
  }

  getImages(name: string): Promise<Image[]> {
    return this.http.get(this.camerasUrl + '/' + name + '/images')
      .toPromise()
      .then(response => response.json)
      .catch(this.handleError);
  }

  private getCameraUrl(name: string): string {
    return this.camerasUrl + '/' + name;
  }

  private handleError(error: any): Promise<any> {
    console.error('An error occurred', error);
    return Promise.reject(error.message || error);
  }
}

function jsonToCameraDetail(json): Camera {
  const camera = json as Camera;
  camera.latest_image.datetime = new Date(camera.latest_image.datetime);
  return camera;
}
