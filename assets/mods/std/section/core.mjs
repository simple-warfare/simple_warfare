class Core {
    constructor({ name, price, mass, buildSpeed, radius, isBio, isBuilder, maxHp }) {
        this.name = name
        this.price = price
        this.mass = mass
        this.buildSpeed = buildSpeed
        this.radius = radius
        this.isBio = isBio
        this.isBuilder = isBuilder
        this.maxHp = maxHp

    }
}


class CoreBuilder {
    constructor() {
        this.params = {
            name: "",
            price: 0,
            mass: 0,
            buildSpeed: 0,
            radius: 0,
            isBio: false,
            isBuilder: false,
            maxHp: 0,
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
        return new Core(this.params)
    }
};


export { CoreBuilder };