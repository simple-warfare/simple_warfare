import { create as transformCreate } from "std:bevy/transform/transform.mjs"
import { CreatedSignal, Signal, UnitEnterSignal, UnitExitSignal } from "std:signal/signal.mjs";
import { TargetType } from "std:sw/sw.mjs";

export class Turret {
    constructor(
        transform,
        image,
        turnSpeed,
        canShoot,
        attackRadius
    ) {
        this.entity = sw.register_entity()
        this.transform = transform
        this.image = image
        this.turnSpeed = turnSpeed
        this.canShoot = canShoot
        this.attackRadius = attackRadius
        this.UnitsInRange = new Array()
        this.unitEnter = new UnitEnterSignal()
        this.unitExit = new UnitExitSignal()
        this.unitEnterSignalEntity = this.unitEnter.entity
        this.unitExitSignalEntity = this.unitExit.entity
        this.created = new CreatedSignal()
    }

    lookAt(target) {
        sw.lookAt(TargetType.Position, this.entity, target)
    }
};

export function fromValues(transform, image, turnSpeed, canShoot, attackRadius) {
    transform = typeof transform !== "undefined" ? transform : transformCreate();
    if (image = typeof image !== "undefined") {
        throw new Error(`must set the image`)
    }
    turnSpeed = typeof turnSpeed !== "undefined" ? turnSpeed : 0.;
    canShoot = typeof canShoot !== "undefined" ? canShoot : true;
    return new Turret(transform, image, turnSpeed, canShoot, attackRadius);
}