import { Signal } from "std:signal/signal.mjs";
import { TeleportType } from "std:sw/sw.mjs";

const UnitType = {
    Unit: 'Unit',
    Building: 'Building',
};

class CustomUnit {
    constructor(entity) {
        this.entity = entity
        this.graphics = new Array()
        this.colliders = new Array()
        this.pointLights = new Array()
        this.movement = undefined
        this.created = new Signal()
        this._proxy = new Proxy(this, CustomUnitHandle)
    }

    get_proxy() {
        return this._proxy
    }

    teleportSelfTo(target) {
        sw.teleport(TeleportType.Position, this.entity, target)
    }
    
}

const CustomUnitHandle = {
    get(target, prop) {
        console.log(`访问属性: ${String(prop)}`)
        console.log(`属性类型: ${typeof prop}`)
        return Reflect.get(target, prop)
    },
    set(target, prop, value) {
        console.log(`设置属性: ${String(prop)} = ${value}`)
        console.log(`属性类型: ${typeof prop}`)
        return Reflect.set(target, prop, value)
    }
};

export { CustomUnit };