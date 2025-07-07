import { fromValues as coreFromValues } from "std:section/core.mjs"
import { CustomUnit } from "std:custom/unit.mjs"
import { fromValues as graphicFromValues } from "std:section/graphic.mjs"
import { fromValues as movementFromValues, MovementType } from "std:section/movement.mjs";
import { CircleCollider, ColliderType } from "std:physics/collider.mjs";
import { Signal } from "std:signal/signal.mjs";
import { fromValues as vec2FromValues } from "package:gl-matrix/vec2.js"
import { SingleTurret } from "custom:example/single_turret.js";

class Tank extends CustomUnit {
    constructor(entity) {
        super(entity)
        this.core = coreFromValues("坦克", 100, 100, 100, 500., 3., 50., true)
        this.graphics.push(graphicFromValues(undefined, "tank.png", undefined, undefined, undefined))
        this.movement = movementFromValues(undefined, 10., 3., 3., 10., 10., 10., 10.)
        this.colliders.push(new CircleCollider(ColliderType.Circle, 25.))
        this.turrets.push(new SingleTurret())
        console.log(JSON.stringify(this.turrets))
        this.created_func = () => {
            this.print_string_when_created.emit("print_string_when_created")
        }
        this.created.connect(this.created_func)

        this.print_string = (string) => {
            console.log(string)
        }

        this.select_func = () => {
            this.teleportSelfTo(vec2FromValues(
                Math.floor(Math.random() * (300 + 1)),
                Math.floor(Math.random() * (300 + 1)),))
        }

        this.selected.connect(this.select_func)
        this.print_string_when_created = new Signal()
        this.print_string_when_created.connect(this.print_string)
    }
}

export { Tank };