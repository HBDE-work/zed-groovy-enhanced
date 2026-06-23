mod config;
mod extension;

pub use zed_extension_api as zed;

use zed_extension_api::register_extension;

use extension::GroovyExtension;

register_extension!(GroovyExtension);
