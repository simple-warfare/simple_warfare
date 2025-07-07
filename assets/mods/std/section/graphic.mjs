import { create as transformCreate } from "std:bevy/transform/transform.mjs"

export class Graphic {
    constructor(
        transform,
        path,
        layer,
        frameWidth,
        frameHeight
    ) {
        this.transform = transform
        this.path = path
        this.layer = layer
        this.frameWidth = frameWidth
        this.frameHeight = frameHeight
    }
}


export function fromValues(transform, path, layer, frameWidth, frameHeight) {
    transform = typeof transform !== "undefined" ? transform : transformCreate();
    path = typeof path !== "undefined" ? path : "";
    layer = typeof layer !== "undefined" ? layer : 0;
    frameWidth = typeof frameWidth !== "undefined" ? frameWidth : 0;
    frameHeight = typeof frameHeight !== "undefined" ? frameHeight : 0;
    return new Graphic(transform, path, layer, frameWidth, frameHeight)
};

