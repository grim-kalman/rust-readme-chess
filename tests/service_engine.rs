use rust_readme_chess::service::engine::{EngineService, EngineResponseDTO};

#[test]
#[ignore]
fn test_engine_service_methods() {
    let mut engine = EngineService::new();
    // These calls should compile; implementation pending
    let _ = engine.start_engine();
    let _ = engine.get_valid_moves();
    let _resp: EngineResponseDTO = engine.get_engine_response();
    let _ = engine.stop_engine();
}