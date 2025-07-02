let create = require("package:gl-vec2-1.3.0/create.js");

class Graphic {
    constructor({ position, path, layer, frameWidth, frameHeight }) {
        this.position = position
        this.path = path
        this.layer = layer
        this.frameWidth = frameWidth
        this.frameHeight = frameHeight
    }
}

class GraphicBuilder {
    constructor() {
        this.params = {
            position: create(),
            path: '',
            layer: 1,
            frameWidth: undefined,
            frameHeight: undefined
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
