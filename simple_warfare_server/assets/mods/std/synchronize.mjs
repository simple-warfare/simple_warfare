export class Synchronize {
  constructor(type) {
    this.type = type;
    this._synchronizeProxy = new Proxy(this, SynchronizeHandle);
    this.entity = simpleWarfareCli.registerEntity(this);
    //this._synchronizeProxy = createDeepProxy(this, SynchronizeHandle)
  }

  getSynchronizeProxy() {
    return this._synchronizeProxy;
  }

  synchronize(data) {
    const keys = Object.keys(data);
    for (const key of keys) {
      this[key] = data[key];
    }
  }
}

export class SynchronizeWithoutEntity {
  constructor(type) {
    this.type = type;
    this._synchronizeProxy = new Proxy(this, SynchronizeHandle);
    //this._synchronizeProxy = createDeepProxy(this, SynchronizeHandle)
  }

  getSynchronizeProxy() {
    return this._synchronizeProxy;
  }

  synchronize(data) {
    const keys = Object.keys(data);
    for (const key of keys) {
      this[key] = data[key];
    }
  }
}

export const SynchronizeType = {
  Core: "Core",
};

export const SynchronizeHandle = {
  get(target, prop) {
    //console.log(`访问属性: ${String(prop)}`)
    //console.log(`属性类型: ${typeof prop}`)
    return Reflect.get(target, prop);
  },
  set(target, prop, value) {
    //console.log(`设置属性: ${String(prop)} = ${value}`)
    //console.log(`属性类型: ${typeof prop}`)
    simpleWarfareCli.synchronize(this.entity, this.type);
    return Reflect.set(target, prop, value);
  },
};

export function createDeepProxy(target, handler) {
  if (typeof target === "object" && target !== null) {
    for (const key in target) {
      target[key] = createDeepProxy(target[key], handler);
    }
    return new Proxy(target, handler);
  }
  return target;
}
