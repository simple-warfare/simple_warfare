import { CustomUnitBuilder, CoreBuilder } from "./simple_warfare_engine.js"

class Tank extends CustomUnitBuilder {
    constructor() {
        super()
        this.name = "Tank"
        let core = new CoreBuilder().with_name("坦克").build()
    };


};

export { Tank };