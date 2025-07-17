import * as creates from "std:create.mjs";
import { CreatedSignal, SelectedSignal } from "std:signal/signal.mjs";
import { TargetType } from "std:sw/sw.mjs";
import { Synchronize } from "std:synchronize.mjs";

export const UnitType = {
    Unit: 'Unit',
    Building: 'Building',
};

export class CustomUnit extends Synchronize {
    constructor() {
        super()
        //this.entity = sw.registerEntity(this)
        this.graphics = new Array()
        this.colliders = new Array()
        this.pointLights = new Array()
        this.turrets = new Array()
        this.movement = undefined
        this.core = creates.core()
        this.created = new CreatedSignal()
        this.selected = new SelectedSignal()
    }

    teleportSelfTo(target) {
        sw.teleport(TargetType.Position, this.entity, target)
    }

    getCore() {
        console.log("getCore")
        return this.core.getSynchronizeProxy()
    }
};
