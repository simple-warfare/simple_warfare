import { CoreBuilder } from "std:section/core.mjs"

class Tank {
    constructor() {
        this.name = "Tank"
        let core = new CoreBuilder().with_name("坦克").build()
    };


};

export { Tank };