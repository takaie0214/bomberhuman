(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/bomberhuman.js":
/*!*****************************!*\
  !*** ../pkg/bomberhuman.js ***!
  \*****************************/
/*! exports provided: GameState, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./bomberhuman_bg.wasm */ \"../pkg/bomberhuman_bg.wasm\");\n/* harmony import */ var _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./bomberhuman_bg.js */ \"../pkg/bomberhuman_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"GameState\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"GameState\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/bomberhuman.js?");

/***/ }),

/***/ "../pkg/bomberhuman_bg.js":
/*!********************************!*\
  !*** ../pkg/bomberhuman_bg.js ***!
  \********************************/
/*! exports provided: GameState, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"GameState\", function() { return GameState; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./bomberhuman_bg.wasm */ \"../pkg/bomberhuman_bg.wasm\");\n\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n/**\n*/\nclass GameState {\n\n    static __wrap(ptr) {\n        const obj = Object.create(GameState.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    free() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_gamestate_free\"](ptr);\n    }\n    /**\n    * Returns a new `GameState` containing a `World` of the given `Size`\n    * @param {number} width\n    * @param {number} height\n    * @returns {GameState}\n    */\n    static new(width, height) {\n        var ret = _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"gamestate_new\"](width, height);\n        return GameState.__wrap(ret);\n    }\n    /**\n    * @param {number} dt\n    */\n    update(dt) {\n        _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"gamestate_update\"](this.ptr, dt);\n    }\n    /**\n    */\n    draw() {\n        _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"gamestate_draw\"](this.ptr);\n    }\n    /**\n    * @param {string} key\n    * @param {number} b\n    */\n    processKey(key, b) {\n        var ptr0 = passStringToWasm0(key, _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        var len0 = WASM_VECTOR_LEN;\n        _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"gamestate_processKey\"](this.ptr, ptr0, len0, b);\n    }\n}\n\nconst __wbindgen_throw = function(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/bomberhuman_bg.js?");

/***/ }),

/***/ "../pkg/bomberhuman_bg.wasm":
/*!**********************************!*\
  !*** ../pkg/bomberhuman_bg.wasm ***!
  \**********************************/
/*! exports provided: memory, __wbg_gamestate_free, gamestate_new, gamestate_update, gamestate_draw, gamestate_processKey, __wbindgen_malloc, __wbindgen_realloc */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./snippets/bomberhuman-3fb6ca739e397268/src/javascript/canvas.js */ \"../pkg/snippets/bomberhuman-3fb6ca739e397268/src/javascript/canvas.js\");\n\n/* harmony import */ var m1 = __webpack_require__(/*! ./bomberhuman_bg.js */ \"../pkg/bomberhuman_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/bomberhuman_bg.wasm?");

/***/ }),

/***/ "../pkg/snippets/bomberhuman-3fb6ca739e397268/src/javascript/canvas.js":
/*!*****************************************************************************!*\
  !*** ../pkg/snippets/bomberhuman-3fb6ca739e397268/src/javascript/canvas.js ***!
  \*****************************************************************************/
/*! exports provided: draw_player, draw_bomb, draw_wall, clear_screen */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"draw_player\", function() { return draw_player; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"draw_bomb\", function() { return draw_bomb; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"draw_wall\", function() { return draw_wall; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"clear_screen\", function() { return clear_screen; });\nconst canvas = document.getElementById(\"bomberhuman-canvas\");\ncanvas.width=750\ncanvas.height=650\nconst ctx = canvas.getContext('2d');\n\nconst resources = () => {\n  let res = {\n    player: document.createElement('canvas'),\n    bomb: document.createElement('canvas'),\n    wall: document.createElement('canvas'),\n  }\n\n  // Player\n  res.player.width = 50;\n  res.player.height = 50;\n  let pCtx = res.player.getContext('2d');\n  pCtx.fillStyle = \"yellow\";\n  pCtx.beginPath();\n  pCtx.arc(25, 25, 25, 0, 2 * Math.PI);\n  pCtx.fill();\n\n  // Bomb\n  res.bomb.width = 50;\n  res.bomb.height = 50;\n  let bCtx = res.bomb.getContext('2d');\n  bCtx.fillStyle = \"green\";\n  bCtx.beginPath();\n  bCtx.arc(25, 25, 25, 0, 2 * Math.PI);\n  bCtx.fill();\n\n  res.wall.width = 50;\n  res.wall.height = 50;\n  let wCtx = res.wall.getContext('2d');\n  wCtx.fillStyle = \"red\";\n  wCtx.beginPath();\n  wCtx.rect(0, 0, 50,50);\n  wCtx.fill();\n  return res;\n};\n\nconst res = resources();\n\nfunction draw_player(x, y) {\n    ctx.drawImage(res.player, x-25, y-25);\n    ctx.fillStyle = \"black\";\n}\n\nfunction draw_bomb(x, y) {\n    ctx.drawImage(res.bomb, x - 25, y - 25);\n    ctx.fillStyle = \"black\";\n}\n\nfunction draw_wall(x, y) {\n    ctx.drawImage(res.wall, x-25 , y-25);\n    ctx.fillStyle = \"black\";\n}\n\nfunction clear_screen() {\n    ctx.fillStyle = \"black\";\n    ctx.fillRect(0, 0, canvas.width, canvas.height);\n}\n\n\n\n//# sourceURL=webpack:///../pkg/snippets/bomberhuman-3fb6ca739e397268/src/javascript/canvas.js?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var bomberhuman__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! bomberhuman */ \"../pkg/bomberhuman.js\");\n\n\nconst gamestate = bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"GameState\"].new(600, 300);\n\ndocument.addEventListener('keydown', e => gamestate.processKey(e.key, true));\ndocument.addEventListener('keyup', e => gamestate.processKey(e.key, false));\n\n\nvar start = null;\nvar prevTimestamp = null;\n\nconst renderLoop = (timestamp) => {\n    // Initialization\n    if (!prevTimestamp) {\n      start = timestamp;\n      prevTimestamp = timestamp;\n      requestAnimationFrame(renderLoop);\n      return;\n    }\n\n    // Update and draw\n    let progress = (timestamp - prevTimestamp) / 1000;\n    gamestate.update(progress); \n    gamestate.draw(); \n\n    // Some bookkeeping\n    prevTimestamp = timestamp;\n    requestAnimationFrame(renderLoop);\n};\n\nrenderLoop();\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ })

}]);