export class Signal {
    constructor() {
        this.entity = sw.register_entity()
        this.connectArray = new Array()
    }

    connect(func) {
        this.connectArray.push(func)
    }

    emit(args) {
        sw.signal_emit(this, args)
    }
};

export const DefaultSignalType = {
    Created: 'Created',
    Selected: 'Selected',
    UnitEnter: "UnitEnter",
    UnitExit: "UnitExit"
};

export class CreatedSignal extends Signal {
    constructor() {
        super()
        this.type = DefaultSignalType.Created
        sw.registerDefaultSignal(this)
    }
};


export class SelectedSignal extends Signal {
    constructor() {
        super()
        this.type = DefaultSignalType.Selected
        sw.registerDefaultSignal(this)
    }
};

export class UnitEnterSignal extends Signal {
    constructor() {
        super()
        this.type = DefaultSignalType.UnitEnter
        sw.registerDefaultSignal(this)
    }
};

export class UnitExitSignal extends Signal {
    constructor() {
        super()
        this.type = DefaultSignalType.UnitExit
        sw.registerDefaultSignal(this)
    }
};