import { Synchronize } from "std:synchronize.mjs";

export const ColliderType = {
  Circle: "Circle",
  Rectangle: "Rectangle",
};

export class CircleCollider extends Synchronize {
  constructor(type, radius) {
    super();
    this.type = type;
    this.radius = radius;
  }
}
export class RectangleCollider extends Synchronize {
  constructor(type, width, height) {
    super();
    this.type = type;
    this.width = width;
    this.height = height;
  }
}
