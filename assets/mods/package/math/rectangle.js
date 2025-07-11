import * as vec2 from "package:gl-matrix/vec2.js";

export class Rectangle {
  /**
   * 创建矩形
   * @param {number} x - 左上角X坐标
   * @param {number} y - 左上角Y坐标
   * @param {number} width - 宽度
   * @param {number} height - 高度
   */
  constructor(x = 0, y = 0, width = 0, height = 0) {
    this._min = vec2.fromValues(x, y);
    this._max = vec2.fromValues(x + width, y + height);
  }

  // ================ 基础属性 ================
  get min() { return vec2.clone(this._min); }
  get max() { return vec2.clone(this._max); }
  
  get x() { return this._min[0]; }
  get y() { return this._min[1]; }
  get width() { return this._max[0] - this._min[0]; }
  get height() { return this._max[1] - this._min[1]; }

  get center() {
    return vec2.fromValues(
      (this._min[0] + this._max[0]) * 0.5,
      (this._min[1] + this._max[1]) * 0.5
    );
  }

  // ================ 几何操作 ================
  /**
   * 检查点是否在矩形内
   * @param {vec2|number[]} point - 要检查的点 [x, y]
   * @returns {boolean}
   */
  containsPoint(point) {
    return point[0] >= this._min[0] && 
           point[0] <= this._max[0] &&
           point[1] >= this._min[1] && 
           point[1] <= this._max[1];
  }

  /**
   * 检查矩形是否完全包含另一个矩形
   * @param {Rectangle} other - 另一个矩形
   * @returns {boolean}
   */
  containsRect(other) {
    return this._min[0] <= other._min[0] &&
           this._max[0] >= other._max[0] &&
           this._min[1] <= other._min[1] &&
           this._max[1] >= other._max[1];
  }

  /**
   * 检查两个矩形是否相交
   * @param {Rectangle} other - 另一个矩形
   * @returns {boolean}
   */
  intersects(other) {
    return !(this._max[0] < other._min[0] ||
             this._min[0] > other._max[0] ||
             this._max[1] < other._min[1] ||
             this._min[1] > other._max[1]);
  }

  /**
   * 计算两个矩形的交集
   * @param {Rectangle} other - 另一个矩形
   * @returns {Rectangle|null} 相交区域或null（如果不相交）
   */
  intersection(other) {
    const minX = Math.max(this._min[0], other._min[0]);
    const minY = Math.max(this._min[1], other._min[1]);
    const maxX = Math.min(this._max[0], other._max[0]);
    const maxY = Math.min(this._max[1], other._max[1]);

    if (maxX < minX || maxY < minY) return null;
    
    return new Rectangle(
      minX,
      minY,
      maxX - minX,
      maxY - minY
    );
  }

  /**
   * 计算两个矩形的并集（包围盒）
   * @param {Rectangle} other - 另一个矩形
   * @returns {Rectangle}
   */
  union(other) {
    const minX = Math.min(this._min[0], other._min[0]);
    const minY = Math.min(this._min[1], other._min[1]);
    const maxX = Math.max(this._max[0], other._max[0]);
    const maxY = Math.max(this._max[1], other._max[1]);
    
    return new Rectangle(
      minX,
      minY,
      maxX - minX,
      maxY - minY
    );
  }

  // ================ 变换操作 ================
  /**
   * 移动矩形
   * @param {vec2|number[]} translation - 移动向量 [x, y]
   */
  translate(translation) {
    vec2.add(this._min, this._min, translation);
    vec2.add(this._max, this._max, translation);
    return this;
  }

  /**
   * 缩放矩形（以原点为中心）
   * @param {vec2|number[]} scale - 缩放因子 [sx, sy]
   */
  scale(scale) {
    const center = this.center;
    
    // 移动到原点
    this.translate(vec2.negate(vec2.create(), center));
    
    // 缩放
    vec2.multiply(this._min, this._min, scale);
    vec2.multiply(this._max, this._max, scale);
    
    // 移回原位置
    this.translate(center);
    return this;
  }

  /**
   * 扩展矩形边界
   * @param {number} amount - 扩展量（所有方向）
   */
  expand(amount) {
    vec2.subtract(this._min, this._min, [amount, amount]);
    vec2.add(this._max, this._max, [amount, amount]);
    return this;
  }

  // ================ 实用方法 ================
  /**
   * 克隆矩形
   * @returns {Rectangle}
   */
  clone() {
    return new Rectangle(
      this._min[0],
      this._min[1],
      this.width,
      this.height
    );
  }

  /**
   * 检查两个矩形是否相等
   * @param {Rectangle} other - 另一个矩形
   * @returns {boolean}
   */
  equals(other) {
    return vec2.exactEquals(this._min, other._min) &&
           vec2.exactEquals(this._max, other._max);
  }

  /**
   * 转换为字符串表示
   * @returns {string}
   */
  toString() {
    return `Rectangle(x: ${this.x}, y: ${this.y}, width: ${this.width}, height: ${this.height})`;
  }

  // ================ 静态方法 ================
  /**
   * 从两个点创建矩形
   * @param {vec2} pointA - 点A
   * @param {vec2} pointB - 点B
   * @returns {Rectangle}
   */
  static fromPoints(pointA, pointB) {
    const min = vec2.min(vec2.create(), pointA, pointB);
    const max = vec2.max(vec2.create(), pointA, pointB);
    return new Rectangle(
      min[0],
      min[1],
      max[0] - min[0],
      max[1] - min[1]
    );
  }

  /**
   * 合并多个矩形
   * @param {Rectangle[]} rects - 矩形数组
   * @returns {Rectangle}
   */
  static merge(rects) {
    if (rects.length === 0) return new Rectangle();
    
    const min = vec2.clone(rects[0]._min);
    const max = vec2.clone(rects[0]._max);
    
    for (let i = 1; i < rects.length; i++) {
      vec2.min(min, min, rects[i]._min);
      vec2.max(max, max, rects[i]._max);
    }
    
    return new Rectangle(
      min[0],
      min[1],
      max[0] - min[0],
      max[1] - min[1]
    );
  }
};
