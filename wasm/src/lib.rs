use wasm_bindgen::prelude::*;

/// Generate HTML from a markdown string following the aemark spec.
///
/// # Exemple
///
/// ```javascript
/// import { markdown } from "@ae_utbm/aemark"
///
/// const result = markdown("This some *text* formatted in __markdown__");
/// console.log(result);
/// // <p>This some <em>text</em> formatted in <u>markdown</u></p>\n
/// ```
#[wasm_bindgen]
pub fn markdown(s: &str) -> String {
    mark::markdown(s)
}
