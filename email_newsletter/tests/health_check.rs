//! tests/health_check.rs

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
}
// Launch our application in the background ~somehow~
async fn spawn_app(){
    let server = email_newsletter::run().expect("Failed to bind address");
	let _ = tokio::spawn(server);
}
