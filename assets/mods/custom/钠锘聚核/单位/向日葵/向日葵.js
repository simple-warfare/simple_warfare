import * as manyFromSectionFiles from "std:from-section-file.mjs";
import { CustomUnit } from "std:custom/unit.mjs";
import { CircleCollider, ColliderType } from "std:physics/collider.mjs";
import { SingleTurret } from "custom:example/tank/single_turret.js";
import { ComfirmDialog } from "std:ui/quick/dialog/comfirm.mjs";

export class Sunflower extends CustomUnit {
  constructor(moduleParentPath) {
    super(moduleParentPath);

    let core = sw.fs.readSectionFile(this, "core.section.toml");
    let main_graphic = sw.fs.readSectionFile(this, "graphics/main.section.toml");
    let movement = sw.fs.readSectionFile(this, "movement.section.toml");

    this.core = manyFromSectionFiles.core(core);
    this.graphics.push(manyFromSectionFiles.graphic(main_graphic));
    this.movement = manyFromSectionFiles.movement(movement);

    this.colliders.push(new CircleCollider(ColliderType.Circle, 25));
  }
}
