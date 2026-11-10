//! The bridge between the core and the interface: each function exported to JavaScript only
//! converts its arguments and its result, the work stays in `stepwise-core`.

use wasm_bindgen::prelude::wasm_bindgen;

/// The version of the core, so that the interface can check which module it loaded.
#[wasm_bindgen(js_name = coreVersion)]
#[must_use]
pub fn core_version() -> String {
    stepwise_core::version().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bridge_gives_the_version_of_the_core() {
        assert_eq!(core_version(), stepwise_core::version());
    }
}
