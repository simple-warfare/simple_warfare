import { Turret } from "std:custom/turret.mjs"
import { fromValues as TransformfromValues } from "std:bevy/transform/transform.mjs";
import { fromValues as Vec3fromValues } from "package:gl-matrix/vec3.js"
class SingleTurret extends Turret {
    constructor() {
        super(TransformfromValues(Vec3fromValues(0., 5., 0.), undefined, undefined),)
    }
}