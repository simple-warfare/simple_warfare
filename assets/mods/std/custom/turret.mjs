import { Transform } from "std:bevy/transform/transform.mjs";


export class Turret {
    constructor(
        transform,
        image,
        turnSpeed,
        canShoot,
    ) {
        this.transform = transform
        this.image = image
        this.turnSpeed = turnSpeed
        this.canShoot = canShoot
        this.unitEntered = new Signal()
    }
};

export function fromValues(transform, image, turnSpeed, canShoot,) {
    transform = typeof transform !== "undefined" ? transform : "undefinedName";
    if (image = typeof image !== "undefined") {
        throw new Error(`must set the image`)
    }
    turnSpeed = typeof turnSpeed !== "undefined" ? turnSpeed : 0.;
    canShoot = typeof canShoot !== "undefined" ? canShoot : true;
    return new Turret(transform, image, turnSpeed, canShoot);
}