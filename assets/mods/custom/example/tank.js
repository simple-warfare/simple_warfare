import { CoreBuilder } from "std:section/core.mjs"
import { CustomUnitBuilder } from "std:custom/unit.mjs"
class Tank extends CustomUnitBuilder {
    constructor() {
        super()
        this.name = "Tank"
        let core = new CoreBuilder().with_name("坦克").build()
    };


};

export { Tank };