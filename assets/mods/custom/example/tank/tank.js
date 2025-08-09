import * as manyFromSectionFiles from "std:from-section-file.mjs";
import { CustomUnit } from "std:custom/unit.mjs";
import { CircleCollider, ColliderType } from "std:physics/collider.mjs";
import { Signal } from "std:signal/signal.mjs";
import { SingleTurret } from "custom:example/tank/single_turret.js";
import { ComfirmDialog } from "std:ui/quick/dialog/comfirm.mjs";
import { stringify } from "package:smol-toml/index.js";

export class Tank extends CustomUnit {
  constructor(moduleParentPath) {
    super(moduleParentPath);

    let core = simpleWarfareCli.fs.readSectionFile(this, "core.toml");
    let main_graphic = simpleWarfareCli.fs.readSectionFile(this, "graphics/main.toml");
    let movement = simpleWarfareCli.fs.readSectionFile(this, "movement.toml");

    this.core = manyFromSectionFiles.core(core);
    this.graphics.push(manyFromSectionFiles.graphic(main_graphic));
    this.movement = manyFromSectionFiles.movement(movement);

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
