import { Synchronize } from "std:synchronize.mjs";

export class Signal extends Synchronize {
  constructor() {
    super();
    this.type = DefaultSignalType.Custom;
    this.entity = simpleWarfareCli.registerEntity(this);
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
