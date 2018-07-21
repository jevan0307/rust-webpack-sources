let wasm = require("./build/webpack_sources");
let StringCache = require("./StringCache");
let SourcesPool = require("./SourcesPool");
exports.Source = require("./Source");

exports.RawSource = require("./RawSource");
exports.OriginalSource = require("./OriginalSource");
exports.SourceMapSource = require("./SourceMapSource");
exports.LineToLineMappedSource = require("./LineToLineMappedSource");

exports.CachedSource = require("./CachedSource");
exports.ConcatSource = require("./ConcatSource");
exports.ReplaceSource = require("./ReplaceSource");
exports.PrefixSource = require("./PrefixSource");

exports.fromStringWithSourceMap = require("./wasm-source-list-map").fromStringWithSourceMap;
exports.SourceListMap = require("./wasm-source-list-map").SourceListMap;
exports.SingleLineNode = require("./wasm-source-list-map").SingleLineNode;
exports.SourceNode = require("./wasm-source-list-map").SourceNode;
exports.CodeNode = require("./wasm-source-list-map").CodeNode;

exports.clear = function() {
    StringCache.clear();
    SourcesPool.clear();
};
