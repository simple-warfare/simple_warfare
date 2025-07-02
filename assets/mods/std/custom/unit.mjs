const UnitType = {
  Unit: 'Unit',
  Building: 'Building',
};

class CustomUnit {
    constructor() {
        this.graphics = new Array()
        this._proxy = new Proxy(this, CustomUnitHandle)
    }

    get_proxy() {
        return this._proxy
    }
}


class CustomUnitBuilder {
    constructor() {

    }
};

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

export { CustomUnit, CustomUnitBuilder };