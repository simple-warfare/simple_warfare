import { CustomUnit, Core } from "./simple_warfare_engine.js"

class Tank extends CustomUnit {
    constructor() {
        let core = new Core("坦克", 1000, 1000)
        super(core)
    };
};

export { Tank };