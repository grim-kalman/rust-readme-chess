use rust_readme_chess::config::AppConfig;
use std::env;

// Combined test: custom and default engine path resolution
#[test]
fn test_app_config_paths_and_defaults() {
    // Clean slate
    unsafe {
        env::remove_var("GITHUB_TOKEN");
        env::remove_var("CHESS_ENGINE_PATH");
    }
    // 1) Custom engine path via env
    unsafe {
        env::set_var("GITHUB_TOKEN", "secret");
        env::set_var("CHESS_ENGINE_PATH", "/custom/engine/path");
    }
    let cfg_custom = AppConfig::new();
    assert_eq!(cfg_custom.token(), "secret");
    assert_eq!(cfg_custom.engine_path(), "/custom/engine/path");
    // 2) Remove custom path, expect fallback
    unsafe { env::remove_var("CHESS_ENGINE_PATH"); }
    let cfg_default = AppConfig::new();
    assert_eq!(cfg_default.engine_path(), "/usr/bin/stockfish");
    // Static getters unchanged
    assert_eq!(cfg_default.github_url(), "https://github.com/grim-kalman");
    assert_eq!(cfg_default.github_api_url(), "https://api.github.com/repos/");
    assert_eq!(cfg_default.readme_path(), "README.md");
    assert_eq!(cfg_default.branch(), "main");
    assert_eq!(cfg_default.owner_repo(), "grim-kalman");
}