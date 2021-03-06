use source_list_map::*;
use types::string_slice::*;
use wasm_api::_SourceListMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn _from_string_with_source_map(
    code: String,
    sources: &[i32],
    sources_content: &[i32],
    mappings: String,
) -> _SourceListMap {
    let sources = sources.to_vec();
    let sources_content = sources_content.to_vec();

    _SourceListMap::new(from_string_with_source_map(
        StringSlice::from(code),
        sources,
        sources_content,
        StringSlice::from(mappings),
    ))
}
