import { Synchronize, SynchronizeWithoutEntity } from "std:synchronize.mjs";

export class Signal extends Synchronize {
  constructor() {
    super();
    this.type = DefaultSignalType.Custom;
    this.connectArray = new Array();
    simpleWarfareCli.registerSignal(this);
  }

  connect(func) {
    this.connectArray.push(func);
  }

  emit(args) {
    simpleWarfareCli.signalEmit(this, args);
  }
}

export const DefaultSignalType = {
  Custom: "Custom",
  Created: "Created",
  Selected: "Selected",
  OnUnitEnter: "OnUnitEnter",
  OnUnitExit: "OnUnitExit",
  NewWayPoint: "NewWayPoint",
  ActiveWayPointChanged: "ActiveWayPointChanged",
};

export class CreatedSignal extends Signal {
  constructor() {
    super();
    this.type = DefaultSignalType.Created;
    simpleWarfareCli.registerDefaultSignal(this);
  }
}

export class SelectedSignal extends Signal {
  constructor() {
    super();
    this.type = DefaultSignalType.Selected;
    simpleWarfareCli.registerDefaultSignal(this);
  }
}

export class NewWayPointSignal extends Signal {
  constructor() {
    super();
    this.type = DefaultSignalType.NewWayPoint;
    simpleWarfareCli.registerDefaultSignal(this);
  }
}

export class ActiveWayPointChangedSignal extends Signal {
  constructor() {
    super();
    this.type = DefaultSignalType.ActiveWayPointChanged;
    simpleWarfareCli.registerDefaultSignal(this);
  }
}

export class OnUnitEnterSignal extends Signal {
  constructor() {
    super();
    this.type = DefaultSignalType.OnUnitEnter;
    simpleWarfareCli.registerDefaultSignal(this);
  }
}

export class OnUnitExitSignal extends Signal {
  constructor() {
    super();
    this.type = DefaultSignalType.OnUnitExit;
    simpleWarfareCli.registerDefaultSignal(this);
  }
}

export class SignalStorage extends SynchronizeWithoutEntity {
  constructor(entity) {
    super();
    this.entity = entity;
    this.signalMap = new Map();
  }
  addSignals(...signals) {
    for (let signal of signals) {
      this.signalMap.set(signal.type, signal);
    }
  }
}
