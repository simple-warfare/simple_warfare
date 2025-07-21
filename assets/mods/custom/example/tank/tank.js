import * as manyFromValues from "std:from-values.mjs"
import { CustomUnit } from "std:custom/unit.mjs"
import { CircleCollider, ColliderType } from "std:physics/collider.mjs";
import { Signal } from "std:signal/signal.mjs";
import { SingleTurret } from "custom:example/tank/single_turret.js";
import { ComfirmDialog } from "std:ui/quick/dialog/comfirm.mjs";
import { parse } from "package:smol-toml/index.js";

export class Tank extends CustomUnit {
    constructor() {
        super()
        this.core = manyFromValues.core("坦克", 100, 100, 100, 500., 3., 50., true)
        this.graphics.push(manyFromValues.graphic(36, 55, "tank.png", undefined, undefined, undefined, undefined, undefined))
        this.movement = manyFromValues.movement(undefined, 10., 7., 3., 10., 10., 10., 10.)
        this.colliders.push(new CircleCollider(ColliderType.Circle, 25.))

        let core_sting = sw.fs.readFile("mods/custom/example/tank/core.toml")

        this.onUnitEnterFunc = () => {
            this.quick_dialog = new ComfirmDialog("New", "new tank")
            this.quick_dialog.onPressComfirm.connect(this.onPressComfirmFunc)
            this.quick_dialog.onPressCancel.connect(this.onPressCancelFunc)
        }
        let turret = new SingleTurret()
        //turret.onUnitEnter.connect(this.onUnitEnterFunc)
        this.turrets.push(turret)

        this.onPressComfirmFunc = () => {
            console.log("Comfirm")
        }
        this.onPressCancelFunc = () => {
            console.log("Cancel")
        }
    }
};