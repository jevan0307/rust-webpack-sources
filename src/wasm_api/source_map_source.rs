use source::SourceTrait;
use source_map_source::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_api::{_MSourceNode, _SourceListMap};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct _SourceMapSource {
    val: Rc<RefCell<SourceMapSource>>,
}

#[wasm_bindgen]
impl _SourceMapSource {
    pub fn _new_string_sidx_string_map(
        value: String,
        value_idx: i32,
        name: i32,
        map_sources: &[i32],
        map_sources_content: &[i32],
        map_mappings: String,
        map_names: &[i32],
    ) -> _SourceMapSource {
        _SourceMapSource {
            val: Rc::new(RefCell::new(SourceMapSource::new(
                value,
                value_idx,
                name,
                map_sources.to_vec(),
                map_sources_content.to_vec(),
                map_mappings,
                map_names.to_vec(),
            ))),
        }
    }

    pub fn _set_original_source_sidx(&mut self, source: i32) {
        self.val.borrow_mut().set_original_source(source);
    }

    pub fn _set_inner_source_map_map(
        &mut self,
        map_sources: &[i32],
        map_sources_content: &[i32],
        map_mappings: String,
        map_names: &[i32],
    ) {
        self.val.borrow_mut().set_inner_source_map(
            map_sources.to_vec(),
            map_sources_content.to_vec(),
            map_mappings,
            map_names.to_vec(),
        );
    }

    pub fn _source(&mut self) -> String {
        (*self.val.borrow_mut().source()).clone()
    }

    pub fn _size(&mut self) -> u32 {
        self.val.borrow_mut().size() as u32
    }

    pub fn _list_map_bool_bool(&mut self, columns: bool, module: bool) -> _SourceListMap {
        _SourceListMap::new(self.val.borrow_mut().list_map(columns, module))
    }

    pub fn _node_bool_bool(&mut self, columns: bool, module: bool) -> _MSourceNode {
        _MSourceNode::new(self.val.borrow_mut().node(columns, module))
    }
}

impl _SourceMapSource {
    pub fn get_raw(&self) -> Rc<RefCell<SourceMapSource>> {
        Rc::clone(&self.val)
    }
}
