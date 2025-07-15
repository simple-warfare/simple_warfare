export class Core {
    constructor(name, hp, maxHp, price, mass, buildSpeed, radius, enablePhysics) {
        this.entity = sw.register_entity(this)
        this.name = name
        this.hp = hp
        this.maxHp = maxHp
        this.price = price
        this.mass = mass
        this.buildSpeed = buildSpeed
        this.radius = radius
        this.enablePhysics = enablePhysics
    }
    synchronize(core){
        this.name = core.name
        this.hp = core.hp
        this.maxHp = core.maxHp
        this.price = core.price
        this.mass = core.mass
        this.buildSpeed = core.buildSpeed
        this.radius = core.radius
        this.enablePhysics = core.enablePhysics
    }
};


export function fromValues(name = "", hp = 0, maxHp = 0, price = 0, mass = 0., buildSpeed = 0., radius = 0., enablePhysics = true) {
    name = typeof name !== "undefined" ? name : "undefinedName";
    hp = typeof hp !== "undefined" ? hp : 0;
    maxHp = typeof maxHp !== "undefined" ? maxHp : 0;
    price = typeof price !== "undefined" ? price : 0;
    mass = typeof mass !== "undefined" ? mass : 0.;
    buildSpeed = typeof buildSpeed !== "undefined" ? buildSpeed : 0.;
    radius = typeof radius !== "undefined" ? radius : 0.;
    enablePhysics = typeof enablePhysics !== "undefined" ? enablePhysics : true;
    return new Core(name, hp, maxHp, price, mass, buildSpeed, radius, enablePhysics);
};
