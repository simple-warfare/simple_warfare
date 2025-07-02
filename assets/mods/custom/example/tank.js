import { CoreBuilder } from "std:section/core.mjs"
import { CustomUnit } from "std:custom/unit.mjs"
import { Graphic } from "std:section/graphic.js"
class Tank extends CustomUnit {
    constructor() {
        super()
        this.name = "Tank"
        let core = new CoreBuilder().withName("坦克").build()
    }


}

export { Tank };