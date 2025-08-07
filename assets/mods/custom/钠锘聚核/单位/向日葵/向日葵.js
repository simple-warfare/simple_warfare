import {
  CustomUnit,
  manyFromSectionFiles,
  CircleCollider,
  ColliderType,
} from "std:index.mjs";

export class Sunflower extends CustomUnit {
  constructor(moduleParentPath) {
    super(moduleParentPath);

    let core = sw.fs.readSectionFile(this, "core.section.toml");
    let main_graphic_file = sw.fs.readSectionFile(
      this,
      "graphics/main.section.toml"
    );
    let movement = sw.fs.readSectionFile(this, "movement.section.toml");

    this.core = manyFromSectionFiles.core(core);

    this.main_graphic = manyFromSectionFiles.graphic(main_graphic_file);

    this.graphics.push(main_graphic);
    this.movement = manyFromSectionFiles.movement(movement);

    console.log(this.graphics[0].easyAnimationRegister[0]);
    this.colliders.push(new CircleCollider(ColliderType.Circle, 25));

    this.newWayPointFunc = (wayPoint) => {
      if (wayPoint.type == "move") {
        this.graphics[0].playTrickFilm()
      }
    };

    this.newWayPoint.connect(this.newWayPointFunc);
  }
}
