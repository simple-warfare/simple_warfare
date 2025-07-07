import { Signal } from "std:signal/signal.mjs";
import { TeleportType } from "std:sw/sw.mjs";

export const UnitType = {
    Unit: 'Unit',
    Building: 'Building',
};

export class CustomUnit {
    constructor(entity) {
        this.entity = entity
        this.graphics = new Array()
        this.colliders = new Array()
        this.pointLights = new Array()
        this.turrets = new Array()
        this.movement = undefined
        this.core = undefined
        this.created = new Signal()
        this._proxy = new Proxy(this, CustomUnitHandle)
    }

    get_proxy() {
        return this._proxy
    }

    teleportSelfTo(target) {
        sw.teleport(TeleportType.Position, this.entity, target)
    }

};

export const CustomUnitHandle = {
    get(target, prop) {
        return Reflect.get(target, prop)
    },
    set(target, prop, value) {
        return Reflect.set(target, prop, value)
    }
};
