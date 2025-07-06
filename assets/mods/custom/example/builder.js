import { CoreBuilder } from "std:section/core.mjs"
import { CustomUnit } from "std:custom/unit.mjs"
import { GraphicBuilder } from "std:section/graphic.mjs"
import { MovementBuilder } from "std:section/movement.mjs";

import { CircleCollider, ColliderType } from "std:physics/collider.mjs";
import { Signal } from "std:signal/signal.mjs";
class Builder extends CustomUnit {
    constructor(entity) {
        super(entity)
        this.graphics = new Array()
        this.colliders = new Array()
        let main_graphics = new GraphicBuilder().withPath("builder.png").withFrameHeight(100).withFrameWidth(100).build()
        this.graphics.push(main_graphics)
        this.movement = new MovementBuilder().withMaxMoveSpeed(3.).build()
        this.name = "Builder"
        let collider = new CircleCollider(ColliderType.Circle, 30.)
        this.colliders.push(collider)
        this.core = new CoreBuilder().withName("建造者").withMass(100.).build()

        this.created_func = () => {
            console.log("created")
            this.print_string_when_created.emit(this.name)
            this.print_string_when_created.emit("print_string_when_created")
        }
        this.created.connect(this.created_func)

        this.print_string = (string) => {
            console.log(JSON.stringify(string))
        }
        this.print_string_when_created = new Signal()
        this.print_string_when_created.connect(this.print_string)



    }
}

export { Builder };