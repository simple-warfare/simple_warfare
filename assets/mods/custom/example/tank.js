import { CoreBuilder } from "std:section/core.mjs"
import { CustomUnit } from "std:custom/unit.mjs"
import { GraphicBuilder } from "std:section/graphic.mjs"
import { MovementBuilder } from "std:section/movement.mjs";


class Tank extends CustomUnit {
    constructor() {
        super()
        this.graphics = new Array()
        let main_graphics = new GraphicBuilder().withPath("tank.png").withFrameHeight(100).withFrameWidth(100).build()
        this.graphics.push(main_graphics)
        this.movement = new MovementBuilder().withMoveSpeed(3.).build()
        this.name = "Tank"
        this.core = new CoreBuilder().withName("坦克").build()
    }
}

export { Tank };