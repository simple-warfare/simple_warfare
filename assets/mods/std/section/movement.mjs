class Movement {
    constructor({ movementType,
        moveSpeed,
        moveAccelerationSpeed,
        moveDecelerationSpeed,
        reverseSpeedPercentage,
        maxTurnSpeed,
        turnAcceleration,
    }) {
        this.movementType = movementType
        this.moveSpeed = moveSpeed
        this.moveAccelerationSpeed = moveAccelerationSpeed
        this.moveDecelerationSpeed = moveDecelerationSpeed
        this.reverseSpeedPercentage = reverseSpeedPercentage
        this.maxTurnSpeed = maxTurnSpeed
        this.turnAcceleration = turnAcceleration

    }
}

const MovementType = {
    LAND: "Land",
};

class MovementBuilder {
    constructor() {
        this.params = {
            movementType: MovementType.LAND,
            moveSpeed: 0.,
            moveAccelerationSpeed: 0.,
            moveDecelerationSpeed: 0.,
            reverseSpeedPercentage: 0.,
            maxTurnSpeed: 0.,
            turnAcceleration: 0.,
        }

        Object.keys(this.params).forEach(key => {
            const methodName = `with${key.charAt(0).toUpperCase() + key.slice(1)}`
            this[methodName] = (value) => {
                this.params[key] = value
                return this
            };
        });
    };

    build() {
        return new Movement(this.params)
    }
};


export { Movement, MovementBuilder, MovementType };