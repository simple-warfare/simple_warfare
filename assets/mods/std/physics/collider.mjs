export const ColliderType = {
    Circle: 'Circle',
    Rectangle: 'Rectangle',
};

export class CircleCollider {
    constructor(type, radius) {
        this.type = type
        this.radius = radius
    }
};
export class RectangleCollider {
    constructor(type, width, height) {
        this.type = type
        this.width = width
        this.height = height
    }
};