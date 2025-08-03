import { Rectangle } from "package:math/rectangle.js";

export class TextureAtlasLayout {
  constructor({ size }) {
    this.size = size;
    this.textures = new Array();
  }
  addTexture(rectangle) {
    this.textures.push(rectangle);
  }
}
