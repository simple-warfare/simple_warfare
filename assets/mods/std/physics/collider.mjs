const ColliderType = {
    Circle: 'Circle',
    Rectangle: 'Rectangle',
};

class CircleCollider {
    constructor(type, radius) {
        this.type = type
        this.radius = radius
    }
}
class RectangleCollider {
    constructor(type, width, height) {
        this.type = type
        this.width = width
        this.height = height
    }
}

export { ColliderType, CircleCollider, RectangleCollider };