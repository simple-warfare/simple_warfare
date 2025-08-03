import { parse } from "package:smol-toml/index.js";
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
    super();
    this.movementType = movementType;
    this.maxMoveSpeed = maxMoveSpeed;
    this.moveAcceleration = moveAcceleration;
    this.moveDeceleration = moveDeceleration;
    this.reversePercentage = reversePercentage;
    this.maxTurnSpeed = maxTurnSpeed;
    this.turnAcceleration = turnAcceleration;
    this.turnDeceleration = turnDeceleration;
    this.angularDamping = angularDamping;
    this.linearDamping = linearDamping;
  }
}

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
  movementType =
    typeof movementType !== "undefined" ? movementType : MovementType.LAND;
  maxMoveSpeed = typeof maxMoveSpeed !== "undefined" ? maxMoveSpeed : 0;
  moveAcceleration =
    typeof moveAcceleration !== "undefined" ? moveAcceleration : 0;
  moveDeceleration =
    typeof moveDeceleration !== "undefined" ? moveDeceleration : 0;
  reversePercentage =
    typeof reversePercentage !== "undefined" ? reversePercentage : 0;
  maxTurnSpeed = typeof maxTurnSpeed !== "undefined" ? maxTurnSpeed : 0;
  turnAcceleration =
    typeof turnAcceleration !== "undefined" ? turnAcceleration : 0;
  turnDeceleration =
    typeof turnDeceleration !== "undefined" ? turnDeceleration : 0;
  return new Movement(
    movementType,
    maxMoveSpeed,
    moveAcceleration,
    moveDeceleration,
    reversePercentage,
    maxTurnSpeed,
    turnAcceleration,
    turnDeceleration
  );
}

export function fromToml(toml) {
  let movement = parse(toml);
  movement.movementType =
    typeof movement.movementType !== "undefined"
      ? movement.movementType
      : MovementType.LAND;
  movement.maxMoveSpeed =
    typeof movement.maxMoveSpeed !== "undefined" ? movement.maxMoveSpeed : 0;
  movement.moveAcceleration =
    typeof movement.moveAcceleration !== "undefined"
      ? movement.moveAcceleration
      : 0;
  movement.moveDeceleration =
    typeof movement.moveDeceleration !== "undefined"
      ? movement.moveDeceleration
      : 0;
  movement.reversePercentage =
    typeof movement.reversePercentage !== "undefined"
      ? movement.reversePercentage
      : 0;
  movement.maxTurnSpeed =
    typeof movement.maxTurnSpeed !== "undefined" ? movement.maxTurnSpeed : 0;
  movement.turnAcceleration =
    typeof movement.turnAcceleration !== "undefined"
      ? movement.turnAcceleration
      : 0;
  movement.turnDeceleration =
    typeof movement.turnDeceleration !== "undefined"
      ? movement.turnDeceleration
      : 0;
  return new Movement(
    movement.movementType,
    movement.maxMoveSpeed,
    movement.moveAcceleration,
    movement.moveDeceleration,
    movement.reversePercentage,
    movement.maxTurnSpeed,
    movement.turnAcceleration,
    movement.turnDeceleration
  );
}
