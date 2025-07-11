import { Turret } from "std:custom/turret.mjs"
import * as manyFromValues from "std:from-values.mjs"
import { TargetType } from "std:sw/sw.mjs";
export class SingleTurret extends Turret {
    constructor() {

        super(manyFromValues.transForm(manyFromValues.vec3(0., 0., 1.), undefined, undefined),
            manyFromValues.graphic(53, 26, "single_turret.png", undefined, undefined, undefined, undefined, manyFromValues.vec2(-10., 0.)), 666., true, 200.)

        this.created_func = () => {
            sw.lookAt(TargetType.Position, this.entity, manyFromValues.vec2(
                Math.floor(Math.random() * (300 + 1)),
                Math.floor(Math.random() * (300 + 1)),))
        }
        this.onUnitEnterFunc = (units) => {
            sw.lookAt(TargetType.Entity, this.entity, units)
        }
        this.onUnitExitFunc = (units) => {
        }
        this.onUnitEnter.connect(this.onUnitEnterFunc)
        this.onUnitExit.connect(this.onUnitExitFunc)
        this.created.connect(this.created_func)
        this.created.emit()
    }
};