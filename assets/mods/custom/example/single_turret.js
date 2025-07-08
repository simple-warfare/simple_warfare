import { Turret } from "std:custom/turret.mjs"
import { fromValues as transFormFromValues } from "std:bevy/transform/transform.mjs";
import { fromValues as vec3FromValues } from "package:gl-matrix/vec3.js"
import { fromValues as graphicFromValues } from "std:section/graphic.mjs"

export class SingleTurret extends Turret {
    constructor() {
        super(transFormFromValues(vec3FromValues(0., 5., 20.), undefined, undefined),
            graphicFromValues (undefined, "single_turret.png", undefined, undefined, undefined),666., true,200.)
    }
};