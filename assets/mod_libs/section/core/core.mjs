class CoreBuilder {
    constructor() {
        this.name = ""
        this.price = 0
        this.mass = 0
        this.build_speed = 0
        this.radius = 0
        this.is_bio = false
        this.isBuilder = false
        this.max_hp = 0

        Object.keys(this).forEach(key => {
            const with_name = `with_${key}`;
            this[with_name] = value => {
                this[key] = value;
                return this;
            }
        })
    };

    build() {
        const keysNoWithers = Object.keys(this).filter(key => typeof this[key] !== 'function');

        return keysNoWithers.reduce((returnValue, key) => {
            return {
                ...returnValue,
                [key]: this[key]
            }
        }, {})
    }
};


export { CoreBuilder };