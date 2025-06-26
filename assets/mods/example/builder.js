import { CustomUnit, Core, Graphics } from "./simple_warfare_engine.js"

class Builder extends CustomUnit {
    constructor() {
        let core = new Core("建造者", 1000, 1000)
        let graphics = new Graphics("builder.png")
        super(core)
    };
};

export { Builder };
