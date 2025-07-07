import { fromValues as CoreFromValues } from "std:section/core.mjs"
import { CustomUnit } from "std:custom/unit.mjs"
import { fromValues as GraphicFromValues } from "std:section/graphic.mjs"
import { fromValues as movementFromValues } from "std:section/movement.mjs";
import { CircleCollider, ColliderType } from "std:physics/collider.mjs";
import { Signal } from "std:signal/signal.mjs";
import { PointLight2d } from "std:section/light2d.mjs";
import Color from "package:color/index.js";
import colors from 'package:color-name/index.js';
import { fromValues as transformFromValues } from "std:bevy/transform/transform.mjs";
import { fromValues as vec3FromValues } from "package:gl-matrix/vec3.js"
import { fromValues as vec2FromValues } from "package:gl-matrix/vec2.js"

class Builder extends CustomUnit {
    constructor(entity) {
        super(entity)
        this.core = CoreFromValues("建造者", 100, 100, 100, 100., 10., 50., true)
        this.graphics.push(GraphicFromValues(undefined, "builder.png", undefined, undefined, undefined))
        this.movement = movementFromValues(undefined,10., 3., 3., 10., 10., 10., 10.)
        this.colliders.push(new CircleCollider(ColliderType.Circle, 30.))

        this.created_func = () => {
            this.teleportSelfTo(vec2FromValues(
                Math.floor(Math.random() * (300 + 1)),
                Math.floor(Math.random() * (300 + 1)),))
            this.print_string_when_created.emit(this.name)
            this.print_string_when_created.emit("print_string_when_created")
        }
        this.created.connect(this.created_func)

        this.print_string = (string) => {
        }
        this.print_string_when_created = new Signal()
        this.print_string_when_created.connect(this.print_string)

        this.pointLights.push(new PointLight2d(transformFromValues(vec3FromValues(10., 0., 0.), undefined, undefined), 50., Color(colors.red), 0.2, 130., false))
        this.pointLights.push(new PointLight2d(transformFromValues(vec3FromValues(-10., 0., 0.), undefined, undefined), 50., Color(colors.red), 0.2, 130., false))

        this.pointLights.push(new PointLight2d(transformFromValues(vec3FromValues(-15., -10., 0.), undefined, undefined), 50., Color(colors.red), 0.2, 130., false))
        this.pointLights.push(new PointLight2d(transformFromValues(vec3FromValues(15., -10., 0.), undefined, undefined), 50., Color(colors.red), 0.2, 130., false))
    }
}

export { Builder };