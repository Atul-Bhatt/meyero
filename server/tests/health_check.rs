use actix_rt;
use reqwest;
use std::net::TcpListener;
use server::run;

#[actix_rt::test]
async fn health_check_works() {
    // run the server and get the address it's running on
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute the request");
    assert!(response.status().is_success());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port");
    
    // get port assigned by the OS
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to run the app.");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}