import { CoreBuilder } from "std:section/core.mjs"
import { CustomUnit } from "std:custom/unit.mjs"
import { GraphicBuilder } from "std:section/graphic.mjs"
import { MovementBuilder } from "std:section/movement.mjs";
import { fromValues } from "package:gl-vec2/fromValues.js";
import { CircleCollider, ColliderType } from "std:physics/collider.mjs";
class Tank extends CustomUnit {
    constructor(entity) {
        super(entity)
        this.graphics = new Array()
        this.colliders = new Array()
        let main_graphics = new GraphicBuilder().withPath("tank.png").withFrameHeight(100).withFrameWidth(100).build()
        this.graphics.push(main_graphics)
        this.movement = new MovementBuilder().withMaxMoveSpeed(3.).build()
        this.name = "Tank"
        this.core = new CoreBuilder().withName("坦克").build()
        let collider = new CircleCollider(ColliderType.Circle, 30.)
        this.colliders.push(collider)
        for (var i = 0; i < 9999; i++) {
            this.teleportSelfTo(fromValues(
                Math.random() * (909 - 10) + 10,
                Math.random() * (909 - 10) + 10))
        }

    }
}

export { Tank };