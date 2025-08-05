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

  setEasyAnimation(easyAnimation) {
    this.easyAnimation = easyAnimation;
  }

  setRealPath(realPath) {
    this.realPath = realPath;
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

export function fromSectionFile(file) {
  let t = parse(file.data);

  t.path = typeof t.path !== "undefined" ? t.path : "";
  t.layer = typeof t.layer !== "undefined" ? t.layer : 0;
  t.offset = typeof t.offset !== "undefined" ? t.offset : create.vec2();

  let graphic = new Graphic(
    t.width,
    t.height,
    t.path,
    t.layer,
    t.frameWidth,
    t.frameHeight,
    t.textureAtlasLayout,
    t.offset
  );

  graphic.setEasyAnimation(t.easyAnimation);
  graphic.setRealPath(file.realPath);
  
  return graphic;
}
