import {
  create as vec3Create,
  fromValues as vec3FromValues,
} from "package:gl-matrix/vec3.js";
import { create as quatCreate } from "package:gl-matrix/quat.js";
export class Transform {
  constructor(translation, rotation, scale) {
    this.translation = translation;
    this.rotation = rotation;
    this.scale = scale;
  }
}

export function create() {
  return new Transform(vec3Create(), quatCreate(), vec3FromValues(1, 1, 1));
}

export function fromValues(translation, rotation, scale) {
  translation = typeof translation !== "undefined" ? translation : vec3Create();
  rotation = typeof rotation !== "undefined" ? rotation : quatCreate();
  scale = typeof scale !== "undefined" ? scale : vec3FromValues(1, 1, 1);
  return new Transform(translation, rotation, scale);
}
