import { create as createVec3, fromValues as fromValuesVec3 } from "package:gl-matrix/vec3.js"
import { create as createQuat } from "package:gl-matrix/quat.js"
class Transform {
    constructor(translation, rotation, scale) {
        this.translation = translation
        this.rotation = rotation
        this.scale = scale
    }
}


export function create() {
    return new Transform(createVec3(), createQuat(), fromValuesVec3(1., 1., 1.))
};

export function fromValues(translation, rotation, scale) {
    translation = typeof translation !== "undefined" ? translation : createVec3();
    rotation = typeof rotation !== "undefined" ? rotation : createQuat();
    scale = typeof scale !== "undefined" ? scale : fromValuesVec3(1., 1., 1.);
    return new Transform(translation, rotation, scale)
};

export { Transform };