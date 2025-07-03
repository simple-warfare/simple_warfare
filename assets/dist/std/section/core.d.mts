export class Core {
    constructor({ name, hp, maxHp, price, mass, buildSpeed, radius, }: {
        name: any;
        hp: any;
        maxHp: any;
        price: any;
        mass: any;
        buildSpeed: any;
        radius: any;
    });
    name: any;
    hp: any;
    maxHp: any;
    price: any;
    mass: any;
    buildSpeed: any;
    radius: any;
}
export class CoreBuilder {
    params: {
        name: string;
        hp: number;
        maxHp: number;
        price: number;
        mass: number;
        buildSpeed: number;
        radius: number;
    };
    build(): Core;
}
//# sourceMappingURL=core.d.mts.map