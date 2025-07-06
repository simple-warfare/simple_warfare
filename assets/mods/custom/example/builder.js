import { CoreBuilder } from "std:section/core.mjs"
import { CustomUnit } from "std:custom/unit.mjs"
import { GraphicBuilder } from "std:section/graphic.mjs"
import { MovementBuilder } from "std:section/movement.mjs";
import { CircleCollider, ColliderType } from "std:physics/collider.mjs";
import { Signal } from "std:signal/signal.mjs";
import { PointLight2d } from "std:section/light2d.mjs";


class Builder extends CustomUnit {
    constructor(entity) {
        super(entity)
        this.core = new CoreBuilder().withName("建造者").withMass(100.).build()
        this.graphics.push(new GraphicBuilder().withPath("builder.png").withFrameHeight(100).withFrameWidth(100).build())
        this.movement = new MovementBuilder().withMaxMoveSpeed(3.).build()
        this.colliders.push(new CircleCollider(ColliderType.Circle, 30.))

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

        this.pointLights.push(new PointLight2d())
    }
}

export { Builder };