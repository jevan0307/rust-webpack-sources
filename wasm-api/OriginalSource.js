"use strict";

const SourceNode = require("./wasm-source-map").SourceNode;
const SourceListMap = require("./wasm-source-list-map").SourceListMap;
const StringCache = require("./StringCache");
const WasmObjectPool = require("./WasmObjectPool");
const Types = require("./Types");
const wasm = require("./build/webpack_sources");

let ptrCache = new Map();
class OriginalSource extends wasm._OriginalSource {
    static _clearPtrCache() {
        ptrCache.clear();
    }

    constructor(value, name) {
        super(0);
        this._value = value;
        this._valueIndex = StringCache.addUnchecked(value);
        this._name = name;
        this._nameIndex = StringCache.add(name);
        if (name === "webpack/bootstrap") {
            let cachedPtr = ptrCache.get(this._valueIndex);
            if (cachedPtr) {
                this.ptr = cachedPtr;
            } else {
                this.ptr = OriginalSource._new_string_sidx_sidx(
                    value,
                    this._valueIndex,
                    this._nameIndex
                ).ptr;
                ptrCache.set(this._valueIndex, this.ptr);
                WasmObjectPool.add(this);
            }
        } else {
            this.ptr = OriginalSource._new_string_sidx_sidx(
                value,
                this._valueIndex,
                this._nameIndex
            ).ptr;
            WasmObjectPool.add(this);
        }
    }

    source() {
        return this._value;
    }

    size() {
        return this._value.length;
    }

    updateHash(hash) {
        hash.update(this._value);
    }
}

require("./SourceAndMapMixin")(OriginalSource.prototype);

OriginalSource.prototype.type = Types.OriginalSource;
module.exports = OriginalSource;
