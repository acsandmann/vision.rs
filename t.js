/* tslint:disable */
/* eslint-disable */

// eslint-disable-next-line @typescript-eslint/no-unused-vars
const { readFileSync } = require('fs')
// eslint-disable-next-line @typescript-eslint/no-unused-vars
const x = require('./@asandmann/vision.node');
const { vnRecognizeTextRequest } = x;
console.log(x)
const rawpng = readFileSync('test.png')
console.log(rawpng.buffer)
console.log(vnRecognizeTextRequest(640, 480, rawpng))

console.info('Simple test passed')
