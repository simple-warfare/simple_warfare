import { manyFromCreate, manyFromValue, SynchronizeWithoutEntity } from "std:index.mjs";
export class Transform extends SynchronizeWithoutEntity {
  constructor(entity, translation, rotation, scale) {
    super()
    this.entity = entity;
    this.translation = translation;
    this.rotation = rotation;
    this.scale = scale;
  }
}

export function create() {
  return new Transform(
    undefined,
    manyFromCreate.vec3(),
    manyFromCreate.quat(),
    manyFromValue.vec3(1, 1, 1)
  );
}

export function fromValues(entity, translation, rotation, scale) {
  translation =
    typeof translation !== "undefined" ? translation : manyFromCreate.vec3()();
  rotation =
    typeof rotation !== "undefined" ? rotation : manyFromCreate.quat()();
  scale = typeof scale !== "undefined" ? scale : manyFromValue.vec3(1, 1, 1);
  return new Transform(entity, translation, rotation, scale);
}
