import { Synchronize } from "std:synchronize.mjs";

class Collider extends Synchronize {
  constructor() {
    super();
    this.collider = new Array();
  }
}

export { Collider };
