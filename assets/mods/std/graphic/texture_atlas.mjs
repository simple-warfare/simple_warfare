import { create } from "package:gl-matrix/vec2.js";

class TextureAtlasLayout {
    constructor({ size }) {
        this.size = size
    }
}


class TextureAtlasLayoutBuilder {
    constructor() {
        this.params = {
            size: create(),
        }

        Object.keys(this.params).forEach(key => {
            const methodName = `with${key.charAt(0).toUpperCase() + key.slice(1)}`
            this[methodName] = (value) => {
                this.params[key] = value
                return this
            };
        });
    }
}
