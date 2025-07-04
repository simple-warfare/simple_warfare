class Core {
    constructor({ name, hp, maxHp, price, mass, buildSpeed, radius, }) {
        this.name = name
        this.hp = hp
        this.maxHp = maxHp
        this.price = price
        this.mass = mass
        this.buildSpeed = buildSpeed
        this.radius = radius

    }
}


class CoreBuilder {
    constructor() {
        this.params = {
            name: "",
            hp: 0,
            maxHp: 0,
            price: 0,
            mass: 0.,
            buildSpeed: 0.,
            radius: 0.,
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


export { Core, CoreBuilder };