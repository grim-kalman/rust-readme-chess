use rust_readme_chess::utils::validator::MoveValidator;
use rust_readme_chess::service::engine::EngineService;

#[test]
fn test_is_valid_and_start_of_valid_move() {
    // Setup a dummy engine service and validator
    let engine = EngineService::new();
    let validator = MoveValidator::new(engine);

    // These calls should compile; implementation pending.
    let _ = validator.is_valid("e2e4");
    let _ = validator.is_start_of_valid_move("e2");
}