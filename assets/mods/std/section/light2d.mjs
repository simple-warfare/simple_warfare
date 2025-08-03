import Color from "package:color/index.js";
import { create as transformCreate } from "std:bevy/transform/transform.mjs";
import { Synchronize } from "std:synchronize.mjs";

export class PointLight2d extends Synchronize {
  constructor(
    transform = transformCreate(),
    radius = 0.5,
    color = Color({ r: 255, g: 255, b: 255, alpha: 0.5 }),
    intensity = 1.0,
    falloff = 0.0,
    cast_shadows = false
  ) {
    super();
    this.transform = transform;
    this.color = color;
    this.radius = radius;
    this.intensity = intensity;
    this.falloff = falloff;
    this.cast_shadows = cast_shadows;
  }
}

export function fromValues(
  transform = create(),
  radius = 0.5,
  color = Color({ r: 255, g: 255, b: 255, alpha: 0.5 }),
  intensity = 1.0,
  falloff = 0.0,
  cast_shadows = false
) {
  return new PointLight2d(
    transform,
    radius,
    color,
    intensity,
    falloff,
    cast_shadows
  );
}
