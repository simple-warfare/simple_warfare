export class CoreBuilder {
    params: {
        name: string;
        price: number;
        mass: number;
        buildSpeed: number;
        radius: number;
        isBio: boolean;
        isBuilder: boolean;
        maxHp: number;
    };
    build(): Core;
}
declare class Core {
    constructor({ name, price, mass, buildSpeed, radius, isBio, isBuilder, maxHp }: {
        name: any;
        price: any;
        mass: any;
        buildSpeed: any;
        radius: any;
        isBio: any;
        isBuilder: any;
        maxHp: any;
    });
    name: any;
    price: any;
    mass: any;
    buildSpeed: any;
    radius: any;
    isBio: any;
    isBuilder: any;
    maxHp: any;
}
export {};
//# sourceMappingURL=core.d.mts.map