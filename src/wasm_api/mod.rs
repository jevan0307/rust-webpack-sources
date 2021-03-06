mod concat_source;
mod line_to_line_mapped_source;
mod original_source;
mod prefix_source;
mod raw_source;
mod replace_source;
mod source_map_source;

mod source_list_map;
mod source_map;
mod wasm_containers;

pub use wasm_api::concat_source::*;
pub use wasm_api::line_to_line_mapped_source::*;
pub use wasm_api::original_source::*;
pub use wasm_api::prefix_source::*;
pub use wasm_api::raw_source::*;
pub use wasm_api::replace_source::*;
pub use wasm_api::source_map_source::*;

pub use wasm_api::source_list_map::*;
pub use wasm_api::source_map::*;
pub use wasm_api::wasm_containers::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console, js_name=log)]
    pub fn clog(s: &str);
}
