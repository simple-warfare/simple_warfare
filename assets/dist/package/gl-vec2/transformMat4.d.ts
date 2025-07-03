export = transformMat4;
/**
 * Transforms the vec2 with a mat4
 * 3rd vector component is implicitly '0'
 * 4th vector component is implicitly '1'
 *
 * @param {vec2} out the receiving vector
 * @param {vec2} a the vector to transform
 * @param {mat4} m matrix to transform with
 * @returns {vec2} out
 */
declare function transformMat4(out: vec2, a: vec2, m: mat4): vec2;
//# sourceMappingURL=transformMat4.d.ts.map