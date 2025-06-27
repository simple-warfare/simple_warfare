import { CustomUnitBuilder, CoreBuilder } from "./simple_warfare_engine.js"

class Tank extends CustomUnitBuilder {
    constructor() {
        let core = new CoreBuilder().with_name("坦克").build()

        console.log(JSON.stringify(core))
        super()
    };
};

export { Tank };