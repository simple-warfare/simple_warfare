export class Graphic {
    constructor({ position, path, layer, frameWidth, frameHeight }: {
        position: any;
        path: any;
        layer: any;
        frameWidth: any;
        frameHeight: any;
    });
    position: any;
    path: any;
    layer: any;
    frameWidth: any;
    frameHeight: any;
}
export class GraphicBuilder {
    params: {
        position: vec2;
        path: string;
        layer: number;
        frameWidth: number;
        frameHeight: number;
    };
    build(): Graphic;
}
//# sourceMappingURL=graphic.d.mts.map