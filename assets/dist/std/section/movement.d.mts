export class Movement {
    constructor({ movementType, moveSpeed, moveAccelerationSpeed, moveDecelerationSpeed, reverseSpeedPercentage, maxTurnSpeed, turnAcceleration, }: {
        movementType: any;
        moveSpeed: any;
        moveAccelerationSpeed: any;
        moveDecelerationSpeed: any;
        reverseSpeedPercentage: any;
        maxTurnSpeed: any;
        turnAcceleration: any;
    });
    movementType: any;
    moveSpeed: any;
    moveAccelerationSpeed: any;
    moveDecelerationSpeed: any;
    reverseSpeedPercentage: any;
    maxTurnSpeed: any;
    turnAcceleration: any;
}
export class MovementBuilder {
    params: {
        movementType: string;
        moveSpeed: number;
        moveAccelerationSpeed: number;
        moveDecelerationSpeed: number;
        reverseSpeedPercentage: number;
        maxTurnSpeed: number;
        turnAcceleration: number;
    };
    build(): Movement;
}
export namespace MovementType {
    let LAND: string;
}
//# sourceMappingURL=movement.d.mts.map