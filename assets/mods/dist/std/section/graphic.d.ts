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
        position: any;
        path: string;
        layer: number;
        frameWidth: any;
        frameHeight: any;
    };
    build(): Graphic;
}
//# sourceMappingURL=graphic.d.ts.map