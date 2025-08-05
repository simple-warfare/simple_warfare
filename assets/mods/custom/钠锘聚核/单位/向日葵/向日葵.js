import * as manyFromTomls from "std:form-toml.mjs";
import { CustomUnit } from "std:custom/unit.mjs";
import { CircleCollider, ColliderType } from "std:physics/collider.mjs";
import { SingleTurret } from "custom:example/tank/single_turret.js";
import { ComfirmDialog } from "std:ui/quick/dialog/comfirm.mjs";

export class Sunflower extends CustomUnit {
  constructor(moduleParentPath) {
    super(moduleParentPath);

    let core = sw.fs.readFile(this, "core.section.toml");
    let main_graphic = sw.fs.readFile(this, "graphics/main.section.toml");
    let movement = sw.fs.readFile(this, "movement.section.toml");

    this.core = manyFromTomls.core(core);
    this.graphics.push(manyFromTomls.graphic(main_graphic));
    this.movement = manyFromTomls.movement(movement);

    this.colliders.push(new CircleCollider(ColliderType.Circle, 25));
  }
}
