use ritual::config::Config;
use ritual::cpp_data::CppPath;
use ritual::rust_info::RustPathScope;
use ritual::rust_type::RustPath;
use ritual_common::errors::Result;

/// QtWebEngine specific configuration.
pub fn web_engine_config(config: &mut Config) -> Result<()> {
    let namespace = CppPath::from_good_str("QtWebEngine");
    config.set_rust_path_scope_hook(move |path| {
        if path == &namespace {
            return Ok(Some(RustPathScope {
                path: RustPath::from_good_str("qt_web_engine"),
                prefix: None,
            }));
        }
        Ok(None)
    });
    Ok(())
}

/// QtWebEngineWidgets specific configuration.
pub fn web_engine_widgets_config(config: &mut Config) -> Result<()> {
    let namespace = CppPath::from_good_str("QtWebEngineWidgets");
    config.set_rust_path_scope_hook(move |path| {
        if path == &namespace {
            return Ok(Some(RustPathScope {
                path: RustPath::from_good_str("qt_web_engine_widgets"),
                prefix: None,
            }));
        }
        Ok(None)
    });
    Ok(())
}
