import {
  manyFromValue,
  CreatedSignal,
  NewWayPointSignal,
  SelectedSignal,
  TargetType,
  Synchronize,
  manyFromCreate,
} from "std:index.mjs";

export const UnitType = {
  Unit: "Unit",
  Building: "Building",
};

export class CustomUnit extends Synchronize {
  constructor(moduleParentPath) {
    super();
    this.moduleParentPath = moduleParentPath;
    this.transform = manyFromCreate.transform();
    this.transform.entity = this.entity;
    //this.entity = simpleWarfareCli.registerEntity(this)
    this.graphics = new Array();
    this.colliders = new Array();
    this.pointLights = new Array();
    this.turrets = new Array();
    this.movement = undefined;
    this.core = manyFromCreate.core();
    this.created = new CreatedSignal();
    this.selected = new SelectedSignal();
    this.newWayPoint = new NewWayPointSignal();
    this.newWayPointEntity = this.newWayPoint.entity;
    //simpleWarfareCli.bindInnerInfo(this);
  }

  teleportSelfTo(target) {
    simpleWarfareCli.teleport(TargetType.Position, this.entity, target);
  }

  getCore() {
    console.log("getCore");
    return this.core.getSynchronizeProxy();
  }
}
