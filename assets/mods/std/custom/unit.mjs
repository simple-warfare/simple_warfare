import { CreatedSignal, SelectedSignal } from "std:signal/signal.mjs";
import { TargetType } from "std:sw/sw.mjs";

export const UnitType = {
    Unit: 'Unit',
    Building: 'Building',
};

export class CustomUnit {
    constructor() {
        this.entity = sw.register_entity()
        this.graphics = new Array()
        this.colliders = new Array()
        this.pointLights = new Array()
        this.turrets = new Array()
        this.movement = undefined
        this.core = undefined
        this.created = new CreatedSignal()
        this.selected = new SelectedSignal()
        this._proxy = new Proxy(this, CustomUnitHandle)
    }

    get_proxy() {
        return this._proxy
    }

    teleportSelfTo(target) {
        sw.teleport(TargetType.Position, this.entity, target)
    }

};

export const CustomUnitHandle = {
    get(target, prop) {
        //console.log(`访问属性: ${String(prop)}`)
        //console.log(`属性类型: ${typeof prop}`)
        return Reflect.get(target, prop)
    },
    set(target, prop, value) {
        //console.log(`设置属性: ${String(prop)} = ${value}`)
        //console.log(`属性类型: ${typeof prop}`)
        return Reflect.set(target, prop, value)
    }
};
