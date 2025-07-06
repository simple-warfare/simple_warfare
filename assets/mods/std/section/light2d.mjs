import Color from "package:color/index.js";

class PointLight2d {
    constructor(
        radius = 0.5,
        color = Color({ r: 255, g: 255, b: 255, alpha: 0.5 }),
        intensity = 1.0,
        falloff = 0.0,
        cast_shadows = false
    ) {
        console.log(JSON.stringify(color))
        this.color = color
        this.radius = radius
        this.intensity = intensity
        this.falloff = falloff
        this.cast_shadows = cast_shadows
    }
}


export { PointLight2d };