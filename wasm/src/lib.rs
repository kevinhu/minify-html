mod utils;

use js_sys::Reflect;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! get_prop {
    ($cfg:expr, $x:literal) => {
        Reflect::get($cfg, &JsValue::from_str($x))
            .ok()
            .and_then(|p| p.as_bool())
            .unwrap_or(false)
    };
}

#[wasm_bindgen]
pub fn minify(code: &[u8], cfg: &JsValue) -> Vec<u8> {
    let cfg = minify_html::Cfg {
        do_not_minify_doctype: get_prop!(cfg, "do_not_minify_doctype"),
        ensure_spec_compliant_unquoted_attribute_values: get_prop!(
            cfg,
            "ensure_spec_compliant_unquoted_attribute_values"
        ),
        keep_closing_tags: get_prop!(cfg, "keep_closing_tags"),
        keep_html_and_head_opening_tags: get_prop!(cfg, "keep_html_and_head_opening_tags"),
        keep_spaces_between_attributes: get_prop!(cfg, "keep_spaces_between_attributes"),
        keep_comments: get_prop!(cfg, "keep_comments"),
        minify_css: get_prop!(cfg, "minify_css"),
        minify_css_level_1: get_prop!(cfg, "minify_css_level_1"),
        minify_css_level_2: get_prop!(cfg, "minify_css_level_2"),
        minify_css_level_3: get_prop!(cfg, "minify_css_level_3"),
        minify_js: get_prop!(cfg, "minify_js"),
        remove_bangs: get_prop!(cfg, "remove_bangs"),
        remove_processing_instructions: get_prop!(cfg, "remove_processing_instructions"),
    };
    minify_html::minify(code, &cfg)
}
