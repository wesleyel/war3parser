use war3parser::war3map_metadata::War3MapMetadata;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys::Uint8Array;

use crate::types::{wasm_image::WasmImage, wasm_w3i::WasmW3i};

#[wasm_bindgen(getter_with_clone)]
pub struct WasmMapInfo {
    pub map_info: Option<WasmW3i>,
    pub images: Vec<WasmImage>,
}

impl From<War3MapMetadata> for WasmMapInfo {
    fn from(metadata: War3MapMetadata) -> Self {
        Self {
            map_info: metadata.map_info.map(WasmW3i::from),
            images: metadata.images.into_iter().map(WasmImage::from).collect(),
        }
    }
}

#[wasm_bindgen]
impl WasmMapInfo {
    #[wasm_bindgen(constructor)]
    pub fn new(buffer: Uint8Array) -> Option<Self> {
        let buffer_vec = buffer.to_vec();

        match War3MapMetadata::from(&buffer_vec) {
            Some(mut metadata) => {
                // safe to unwrap because it is ok to fail
                metadata.update_string_table().unwrap_or(());
                Some(Self::from(metadata))
            }
            None => None,
        }
    }
}
