export class Camera {
  name: string;
  description: string;
  latest_image: Image;
  interval: number;
}

export class Image {
  url: string;
  datetime: Date;
}
