import * as create from "std:create.mjs";
import { parse } from "package:smol-toml/index.js";
import { Synchronize } from "std:synchronize.mjs";
export class Graphic extends Synchronize {
  constructor(
    width,
    height,
    path,
    layer,
    frameWidth,
    frameHeight,
    textureAtlasLayout,
    offset
  ) {
    super();
    this.width = width;
    this.height = height;
    this.path = path;
    this.layer = layer;
    this.frameWidth = frameWidth;
    this.frameHeight = frameHeight;
    this.textureAtlasLayout = textureAtlasLayout;
    this.offset = offset;
  }
}

export function fromValues(
  width,
  height,
  path,
  layer,
  frameWidth,
  frameHeight,
  textureAtlasLayout,
  offset
) {
  path = typeof path !== "undefined" ? path : "";
  layer = typeof layer !== "undefined" ? layer : 0;
  offset = typeof offset !== "undefined" ? offset : create.vec2();
  return new Graphic(
    width,
    height,
    path,
    layer,
    frameWidth,
    frameHeight,
    textureAtlasLayout,
    offset
  );
}

export function fromToml(tomlString) {
  let graphic = parse(tomlString);

  graphic.path = typeof graphic.path !== "undefined" ? graphic.path : "";
  graphic.layer = typeof graphic.layer !== "undefined" ? graphic.layer : 0;
  graphic.offset =
    typeof graphic.offset !== "undefined" ? graphic.offset : create.vec2();
  return new Graphic(
    graphic.width,
    graphic.height,
    graphic.path,
    graphic.layer,
    graphic.frameWidth,
    graphic.frameHeight,
    graphic.textureAtlasLayout,
    graphic.offset
  );
}
