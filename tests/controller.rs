use rust_readme_chess::controller::Controller;

#[test]
#[ignore]
fn test_controller_endpoints() {
    let ctrl = Controller::new(/* dependencies */);
    let _ = ctrl.play("e2e4");
    let _ = ctrl.select("e2");
    let _ = ctrl.new_game();
}