import { create as transformCreate } from "std:bevy/transform/transform.mjs";
import {
  CreatedSignal,
  Signal,
  OnUnitEnterSignal,
  OnUnitExitSignal,
} from "std:signal/signal.mjs";
import { TargetType } from "std:sw/sw.mjs";
import { Synchronize } from "std:synchronize.mjs";

export class Turret extends Synchronize {
  constructor(transform, image, turnSpeed, canShoot, attackRadius) {
    super();
    this.transform = transform;
    this.image = image;
    this.turnSpeed = turnSpeed;
    this.canShoot = canShoot;
    this.attackRadius = attackRadius;
    this.UnitsInRange = new Array();
    this.onUnitEnter = new OnUnitEnterSignal();
    this.onUnitExit = new OnUnitExitSignal();
    this.OnUnitEnterSignalEntity = this.onUnitEnter.entity;
    this.OnUnitExitSignalEntity = this.onUnitExit.entity;
    this.created = new CreatedSignal();
  }

  lookAt(target) {
    sw.lookAt(TargetType.Position, this.entity, target);
  }
}

export function fromValues(
  transform,
  image,
  turnSpeed,
  canShoot,
  attackRadius
) {
  transform = typeof transform !== "undefined" ? transform : transformCreate();
  if ((image = typeof image !== "undefined")) {
    throw new Error(`must set the image`);
  }
  turnSpeed = typeof turnSpeed !== "undefined" ? turnSpeed : 0;
  canShoot = typeof canShoot !== "undefined" ? canShoot : true;
  return new Turret(transform, image, turnSpeed, canShoot, attackRadius);
}
