import { CoreBuilder } from "std:section/core.mjs"
import { CustomUnit } from "std:custom/unit.mjs"
import { GraphicBuilder } from "std:section/graphic.mjs"
import { MovementBuilder } from "std:section/movement.mjs";
class Builder extends CustomUnit {
    constructor(entity) {
        super(entity)
        this.graphics = new Array()
        let main_graphics = new GraphicBuilder().withPath("builder.png").withFrameHeight(100).withFrameWidth(100).build()
        this.graphics.push(main_graphics)
        this.movement = new MovementBuilder().withMaxMoveSpeed(3.).build()
        this.name = "Tank"
        this.core = new CoreBuilder().withName("建造者").build()
    }
}

export { Builder };