import { CustomUnit, Core, Graphics } from "./simple_warfare_engine.js"

class Tank extends CustomUnit {
    constructor() {
        let core = new Core("坦克", 1000, 1000)
        let graphics = new Graphics("tank.png")
        super(core)
    };
};

export { Tank };
