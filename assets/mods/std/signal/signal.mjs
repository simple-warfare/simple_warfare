export class Signal {
    constructor() {
        this.entity = sw.register_entity()
        this.connectArray = new Array()
        sw.register_signal(this)
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
    OnUnitEnter: "OnUnitEnter",
    OnUnitExit: "OnUnitExit"
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

export class OnUnitEnterSignal extends Signal {
    constructor() {
        super()
        this.type = DefaultSignalType.OnUnitEnter
        sw.registerDefaultSignal(this)
    }
};

export class OnUnitExitSignal extends Signal {
    constructor() {
        super()
        this.type = DefaultSignalType.OnUnitExit
        sw.registerDefaultSignal(this)
    }
};