use std::path::PathBuf;

use zed_extension_api as zed;

struct Adonix4GL {
    placeholder: Option<PathBuf>,
}

impl zed::Extension for Adonix4GL {
    fn new() -> Self {
        Self { placeholder: None }
    }
}

zed::register_extension!(Adonix4GL);
