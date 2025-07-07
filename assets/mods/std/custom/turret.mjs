import { create as transformCreate } from "std:bevy/transform/transform.mjs"
import { Signal } from "std:signal/signal.mjs";

export class Turret {
    constructor(
        transform,
        image,
        turnSpeed,
        canShoot,
    ) {
        console.log(turnSpeed)
        this.transform = transform
        this.image = image
        this.turnSpeed = turnSpeed
        this.canShoot = canShoot
        this.unitEntered = new Signal()
    }
};

export function fromValues(transform, image, turnSpeed, canShoot) {
    transform = typeof transform !== "undefined" ? transform : transformCreate();
    if (image = typeof image !== "undefined") {
        throw new Error(`must set the image`)
    }
    turnSpeed = typeof turnSpeed !== "undefined" ? turnSpeed : 0.;
    canShoot = typeof canShoot !== "undefined" ? canShoot : true;
    return new Turret(transform, image, turnSpeed, canShoot);
}