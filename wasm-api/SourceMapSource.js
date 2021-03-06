"use strict";

const SourceNode = require("./wasm-source-map").SourceNode;
const SourceListMap = require("./wasm-source-list-map").SourceListMap;
const fromStringWithSourceMap = require("./wasm-source-list-map")
    .fromStringWithSourceMap;
const StringCache = require("./StringCache");
const WasmObjectPool = require("./WasmObjectPool");
const Types = require("./Types");
const wasm = require("./build/webpack_sources");

class SourceMapSource extends wasm._SourceMapSource {
    constructor(value, name, sourceMap, originalSource, innerSourceMap) {
        super(0);
        this._value = value;
        this._valueIndex = StringCache.addUnchecked(value);
        this._name = name;
        this._nameIndex = StringCache.add(name);
        this._sourceMap = sourceMap;

        if (sourceMap._wasmObj) {
            this.ptr = SourceMapSource._new_string_sidx_string_wasmmap(
                value,
                this._valueIndex,
                this._nameIndex,
                sourceMap._wasmObj
            ).ptr;
        } else {
            let sources = (sourceMap.sources || []).map(StringCache.add);
            let sourcesContent = (sourceMap.sourcesContent || []).map(
                StringCache.addUnchecked
            );
            let mappings = sourceMap.mappings;
            let names = (sourceMap.names || []).map(StringCache.add);
            this.ptr = SourceMapSource._new_string_sidx_string_map(
                value,
                this._valueIndex,
                this._nameIndex,
                sources,
                sourcesContent,
                mappings,
                names
            ).ptr;
        }

        if (originalSource) {
            this._originalSource = originalSource;
            this._set_original_source_sidx(
                StringCache.addUnchecked(originalSource)
            );
        }
        if (innerSourceMap) {
            this._innerSourceMap = innerSourceMap;
            if (innerSourceMap._wasmObj) {
                this._set_inner_source_map_wasmmap(innerSourceMap._wasmObj);
            } else {
                let innerSources = (innerSourceMap.sources || []).map(
                    StringCache.add
                );
                let innerSourcesContent = (
                    innerSourceMap.sourcesContent || []
                ).map(StringCache.addUnchecked);
                let innerMappings = innerSourceMap.mappings;
                let innerNames = (innerSourceMap.names || []).map(
                    StringCache.add
                );
                this._set_inner_source_map_map(
                    innerSources,
                    innerSourcesContent,
                    innerMappings,
                    innerNames
                );
            }
        }
        WasmObjectPool.add(this);
    }

    source() {
        return this._value;
    }

    size() {
        return this._value.length;
    }

    updateHash(hash) {
        hash.update(this._value);
        if (this._originalSource) {
            hash.update(this._originalSource);
        }
    }
}

require("./SourceAndMapMixin")(SourceMapSource.prototype);

SourceMapSource.prototype.type = Types.SourceMapSource;
module.exports = SourceMapSource;
