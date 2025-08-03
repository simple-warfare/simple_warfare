import { parse } from "package:smol-toml/index.js";
import { Synchronize } from "std:synchronize.mjs";

export class Core extends Synchronize {
  constructor(name, hp, maxHp, price, mass, buildSpeed, radius, enablePhysics) {
    super();
    this.name = name;
    this.hp = hp;
    this.maxHp = maxHp;
    this.price = price;
    this.mass = mass;
    this.buildSpeed = buildSpeed;
    this.radius = radius;
    this.enablePhysics = enablePhysics;
  }
}

export function fromValues(
  name = "",
  hp = 0,
  maxHp = 0,
  price = 0,
  mass = 0,
  buildSpeed = 0,
  radius = 0,
  enablePhysics = true
) {
  name = typeof name !== "undefined" ? name : "undefinedName";
  hp = typeof hp !== "undefined" ? hp : 0;
  maxHp = typeof maxHp !== "undefined" ? maxHp : 0;
  price = typeof price !== "undefined" ? price : 0;
  mass = typeof mass !== "undefined" ? mass : 0;
  buildSpeed = typeof buildSpeed !== "undefined" ? buildSpeed : 0;
  radius = typeof radius !== "undefined" ? radius : 0;
  enablePhysics = typeof enablePhysics !== "undefined" ? enablePhysics : true;
  return new Core(
    name,
    hp,
    maxHp,
    price,
    mass,
    buildSpeed,
    radius,
    enablePhysics
  );
}

export function create() {
  return new Core("None", 0, 0, 0, 0, 0, 0, true);
}

export function fromToml(toml) {
  let core = parse(toml);
  core.name = typeof core.name !== "undefined" ? core.name : "undefinedName";
  core.hp = typeof core.hp !== "undefined" ? core.hp : 0;
  core.maxHp = typeof core.maxHp !== "undefined" ? core.maxHp : 0;
  core.price = typeof core.price !== "undefined" ? core.price : 0;
  core.mass = typeof core.mass !== "undefined" ? core.mass : 0;
  core.buildSpeed =
    typeof core.buildSpeed !== "undefined" ? core.buildSpeed : 0;
  core.radius = typeof core.radius !== "undefined" ? core.radius : 0;
  core.enablePhysics =
    typeof core.enablePhysics !== "undefined" ? core.enablePhysics : true;

  return new Core(
    core.name,
    core.hp,
    core.maxHp,
    core.price,
    core.mass,
    core.buildSpeed,
    core.radius,
    core.enablePhysics
  );
}
