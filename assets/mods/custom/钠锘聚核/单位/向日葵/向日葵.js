import * as manyFromValues from "std:from-values.mjs";
import * as manyFromTomls from "std:form-toml.mjs";
import { CustomUnit } from "std:custom/unit.mjs";
import { CircleCollider, ColliderType } from "std:physics/collider.mjs";
import { Signal } from "std:signal/signal.mjs";
import { SingleTurret } from "custom:example/tank/single_turret.js";
import { ComfirmDialog } from "std:ui/quick/dialog/comfirm.mjs";
import { stringify } from "package:smol-toml/index.js";

export class Sunflower extends CustomUnit {
  constructor() {
    super();

    let core = sw.fs.readFile(this,"mods/custom/example/tank/core.toml");
    let main_graphic = sw.fs.readFile(
      "mods/custom/example/tank/graphics/main.toml"
    );
    let movement = sw.fs.readFile(this,"mods/custom/example/tank/movement.toml");

    this.core = manyFromTomls.core(core);
    this.graphics.push(manyFromTomls.graphic(main_graphic));
    this.movement = manyFromTomls.movement(movement);

    this.colliders.push(new CircleCollider(ColliderType.Circle, 25));

    this.onUnitEnterFunc = () => {
      this.quick_dialog = new ComfirmDialog("New", "new tank");
      this.quick_dialog.onPressComfirm.connect(this.onPressComfirmFunc);
      this.quick_dialog.onPressCancel.connect(this.onPressCancelFunc);
    };
    let turret = new SingleTurret();
    //turret.onUnitEnter.connect(this.onUnitEnterFunc)
    this.turrets.push(turret);

    this.onPressComfirmFunc = () => {
      console.log("Comfirm");
    };
    this.onPressCancelFunc = () => {
      console.log("Cancel");
    };
  }
}
