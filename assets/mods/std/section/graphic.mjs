import * as create from "std:create.mjs"
export class Graphic {
    constructor(
        width,
        height,
        path,
        layer,
        frameWidth,
        frameHeight,
        textureAtlasLayout,
        offset,
    ) {
        this.width = width
        this.height = height
        this.path = path
        this.layer = layer
        this.frameWidth = frameWidth
        this.frameHeight = frameHeight
        this.textureAtlasLayout = textureAtlasLayout
        this.offset = offset
    }
}


export function fromValues(width, height, path, layer, frameWidth, frameHeight, textureAtlasLayout, offset) {
    path = typeof path !== "undefined" ? path : "";
    layer = typeof layer !== "undefined" ? layer : 0;
    offset = typeof offset !== "undefined" ? offset : create.vec2();
    return new Graphic(width, height, path, layer, frameWidth, frameHeight, textureAtlasLayout, offset)
};

