class CustomUnitBuilder {
    constructor() {
        this._proxy = new Proxy(this, CustomUnitHandle);
    }

    // 始终返回同一个 Proxy 实例
    get_proxy() {
        return this._proxy;
    }
};

const CustomUnitHandle = {
    get(target, prop) {
        console.log(`访问属性: ${String(prop)}`);
        return Reflect.get(target, prop);
    },
    set(target, prop, value) {
        console.log(`设置属性: ${String(prop)} = ${value}`);
        return Reflect.set(target, prop, value);
    }
};

export { CustomUnitBuilder };