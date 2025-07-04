class Movement {
    constructor({ movementType,
        maxMoveSpeed,
        moveAcceleration,
        moveDeceleration,
        reversePercentage,
        maxTurnSpeed,
        turnAcceleration,
        turnDeceleration,
    }) {
        this.movementType = movementType
        this.maxMoveSpeed = maxMoveSpeed
        this.moveAcceleration = moveAcceleration
        this.moveDeceleration = moveDeceleration
        this.reversePercentage = reversePercentage
        this.maxTurnSpeed = maxTurnSpeed
        this.turnAcceleration = turnAcceleration
        this.turnDeceleration = turnDeceleration
    }
}

const MovementType = {
    LAND: "Land",
};

class MovementBuilder {
    constructor() {
        this.params = {
            movementType: MovementType.LAND,
            maxMoveSpeed: 0.,
            moveAcceleration: 0.,
            moveDeceleration: 0.,
            reversePercentage: 0.,
            maxTurnSpeed: 0.,
            turnAcceleration: 0.,
            turnDeceleration: 0.,
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