import { Turret } from "std:custom/turret.mjs"
import { fromValues as transFormFromValues } from "std:bevy/transform/transform.mjs";
import { fromValues as vec3FromValues } from "package:gl-matrix/vec3.js"
import { fromValues as graphicFromValues } from "std:section/graphic.mjs"
import { TargetType } from "std:sw/sw.mjs";
import { fromValues as vec2FromValues } from "package:gl-matrix/vec2.js"
export class SingleTurret extends Turret {
    constructor() {
        super(transFormFromValues(vec3FromValues(0., 0., 1.), undefined, undefined),
            graphicFromValues(undefined, "single_turret.png", undefined, undefined, undefined), 666., true, 200.)

        this.created_func = () => {
            sw.lookAt(TargetType.Position, this.entity, vec2FromValues(
                Math.floor(Math.random() * (300 + 1)),
                Math.floor(Math.random() * (300 + 1)),))
        }
        this.unitEnterFunc = (units) => {
            console.log(`Enter: ${JSON.stringify(units)}`)
        }
        this.unitExitFunc = (units) => {
            console.log(`Exit: ${JSON.stringify(units)}`)
        }
        this.unitEnter.connect(this.unitEnterFunc)
        this.unitExit.connect(this.unitExitFunc)
        this.created.connect(this.created_func)
        this.created.emit()
    }
};