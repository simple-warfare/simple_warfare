import { create } from "std:bevy/transform/transform.mjs"

class Graphic {
    constructor({ transform = create(), path, layer, frameWidth, frameHeight }) {
        this.transform = transform
        this.path = path
        this.layer = layer
        this.frameWidth = frameWidth
        this.frameHeight = frameHeight
    }
}

class GraphicBuilder {
    constructor() {
        this.params = {
            transform: create(),
            path: '',
            layer: 1,
            frameWidth: 0,
            frameHeight: 0
        }

        Object.keys(this.params).forEach(key => {
            const methodName = `with${key.charAt(0).toUpperCase() + key.slice(1)}`
            this[methodName] = (value) => {
                this.params[key] = value
                return this
            }
        })
    }

    build() {
        return new Graphic(this.params)
    }
}

export { Graphic, GraphicBuilder };
