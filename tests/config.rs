use rust_readme_chess::config::AppConfig;
use std::env;

#[test]
fn test_app_config_accessors() {
    // Provide environment variables for secret config (unsafe on some platforms)
    unsafe { env::set_var("GITHUB_TOKEN", "secret"); }
    unsafe { env::set_var("CHESS_ENGINE_PATH", "/usr/local/bin/stockfish"); }
    let cfg = AppConfig::new();
    // Static endpoints
    assert_eq!(cfg.github_url(), "https://github.com/grim-kalman");
    assert_eq!(cfg.github_api_url(), "https://api.github.com/repos/");
    assert_eq!(cfg.readme_path(), "README.md");
    assert_eq!(cfg.branch(), "main");
    assert_eq!(cfg.owner_repo(), "grim-kalman");
    // Environment-driven values
    assert_eq!(cfg.token(), "secret");
    assert_eq!(cfg.engine_path(), "/usr/local/bin/stockfish");
}