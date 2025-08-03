import * as manyFromValues from "std:from-values.mjs";
import { CustomUnit } from "std:custom/unit.mjs";
import { MovementType } from "std:section/movement.mjs";
import { CircleCollider, ColliderType } from "std:physics/collider.mjs";
import { Signal } from "std:signal/signal.mjs";
import { PointLight2d } from "std:section/light2d.mjs";
import Color from "package:color/index.js";
import colors from "package:color-name/index.js";

class Builder extends CustomUnit {
  constructor() {
    super();
    this.core = manyFromValues.core(
      "建造者",
      100,
      100,
      100,
      100,
      100,
      100,
      true
    );
    this.graphics.push(
      manyFromValues.graphic(
        undefined,
        "builder.png",
        undefined,
        undefined,
        undefined
      )
    );
    this.movement = manyFromValues.movement(
      MovementType.LAND,
      10,
      3,
      3,
      10,
      10,
      10,
      10
    );
    this.colliders.push(new CircleCollider(ColliderType.Circle, 20));

    this.created_func = () => {
      this.teleportSelfTo(
        manyFromValues.vec2(
          Math.floor(Math.random() * (300 + 1)),
          Math.floor(Math.random() * (300 + 1))
        )
      );
      this.print_string_when_created.emit(this.name);
      this.print_string_when_created.emit("print_string_when_created");
    };
    this.created.connect(this.created_func);

    this.print_string = (string) => {};
    this.print_string_when_created = new Signal();
    this.print_string_when_created.connect(this.print_string);

    this.pointLights.push(
      new PointLight2d(
        manyFromValues.transform(
          manyFromValues.vec3(10, 0, 0),
          undefined,
          undefined
        ),
        50,
        Color(colors.red),
        0.2,
        130,
        false
      )
    );
    this.pointLights.push(
      new PointLight2d(
        manyFromValues.transform(
          manyFromValues.vec3(-10, 0, 0),
          undefined,
          undefined
        ),
        50,
        Color(colors.red),
        0.2,
        130,
        false
      )
    );

    this.pointLights.push(
      new PointLight2d(
        manyFromValues.transform(
          manyFromValues.vec3(-15, -10, 0),
          undefined,
          undefined
        ),
        50,
        Color(colors.red),
        0.2,
        130,
        false
      )
    );
    this.pointLights.push(
      new PointLight2d(
        manyFromValues.transform(
          manyFromValues.vec3(15, -10, 0),
          undefined,
          undefined
        ),
        50,
        Color(colors.red),
        0.2,
        130,
        false
      )
    );
  }
}

export { Builder };
