import { Synchronize } from "std:synchronize.mjs";

export const MovementType = {
    LAND: "Land",
};

export class Movement extends Synchronize {
    constructor(
        movementType,
        maxMoveSpeed,
        moveAcceleration,
        moveDeceleration,
        reversePercentage,
        maxTurnSpeed,
        turnAcceleration,
        turnDeceleration,
        angularDamping,
        linearDamping
    ) {
        super()
        this.movementType = movementType
        this.maxMoveSpeed = maxMoveSpeed
        this.moveAcceleration = moveAcceleration
        this.moveDeceleration = moveDeceleration
        this.reversePercentage = reversePercentage
        this.maxTurnSpeed = maxTurnSpeed
        this.turnAcceleration = turnAcceleration
        this.turnDeceleration = turnDeceleration
        this.angularDamping = angularDamping
        this.linearDamping = linearDamping
    }
};

export function fromValues(
    movementType,
    maxMoveSpeed,
    moveAcceleration,
    moveDeceleration,
    reversePercentage,
    maxTurnSpeed,
    turnAcceleration,
    turnDeceleration,
    angularDamping,
    linearDamping
) {
    movementType = typeof movementType !== "undefined" ? movementType : MovementType.LAND;
    maxMoveSpeed = typeof maxMoveSpeed !== "undefined" ? maxMoveSpeed : 0.;
    moveAcceleration = typeof moveAcceleration !== "undefined" ? moveAcceleration : 0.;
    moveDeceleration = typeof moveDeceleration !== "undefined" ? moveDeceleration : 0.;
    reversePercentage = typeof reversePercentage !== "undefined" ? reversePercentage : 0.;
    maxTurnSpeed = typeof maxTurnSpeed !== "undefined" ? maxTurnSpeed : 0.;
    turnAcceleration = typeof turnAcceleration !== "undefined" ? turnAcceleration : 0.;
    turnDeceleration = typeof turnDeceleration !== "undefined" ? turnDeceleration : 0.;
    return new Movement(movementType, maxMoveSpeed, moveAcceleration, moveDeceleration, reversePercentage, maxTurnSpeed, turnAcceleration, turnDeceleration)
};
