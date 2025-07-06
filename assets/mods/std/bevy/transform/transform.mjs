import { create as create_vec2 } from "package:gl-matrix/vec3.js"
import { create as create_quat } from "package:gl-matrix/quat.js"
class Transform {
    constructor(translation = create_vec2(), rotation = create_quat(), scale = create_vec2()) {
        this.translation = translation
        this.rotation = rotation
        this.scale = scale
    }
}


export { Transform };