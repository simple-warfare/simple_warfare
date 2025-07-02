export class Graphic {
    constructor({ path, layer, frameWidth, frameHeight }: {
        path: any;
        layer: any;
        frameWidth: any;
        frameHeight: any;
    });
    path: any;
    layer: any;
    frameWidth: any;
    frameHeight: any;
}
export class GraphicBuilder {
    params: {
        path: string;
        layer: number;
        frameWidth: any;
        frameHeight: any;
    };
    build(): Graphic;
}
//# sourceMappingURL=graphic.d.mts.map