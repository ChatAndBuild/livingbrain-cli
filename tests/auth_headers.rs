//! Runs the built `livingbrain` binary against a local mock server and checks
//! what actually goes over the wire for an authenticated command.

use std::process::Command;

use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const API_KEY: &str = "lb_test_key";
const SUBJECT_ID: &str = "3f2b8c1e-7a4d-4e9b-9c2a-5d6e7f8a9b0c";

fn livingbrain(base_url: &str) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_livingbrain"));
    cmd.env_clear()
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .env("HOME", std::env::temp_dir())
        .env("LIVINGBRAIN_NO_KEYRING", "1")
        .current_dir(std::env::temp_dir())
        .args(["--base-url", base_url, "--format", "json"]);
    cmd
}

async fn mock_brains_list() -> MockServer {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/brains"))
        .and(header("x-api-key", API_KEY))
        .and(header("x-subject-id", SUBJECT_ID))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .expect(1)
        .mount(&server)
        .await;
    server
}

fn assert_success(output: std::process::Output) {
    assert!(
        output.status.success(),
        "livingbrain exited with {}\nstdout: {}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

async fn assert_single_request_with_both_headers(server: &MockServer) {
    let requests = server.received_requests().await.unwrap_or_default();
    assert_eq!(
        requests.len(),
        1,
        "expected exactly one request, got {}",
        requests.len()
    );
    let headers = &requests[0].headers;
    assert_eq!(
        headers.get("x-api-key").and_then(|v| v.to_str().ok()),
        Some(API_KEY)
    );
    assert_eq!(
        headers.get("x-subject-id").and_then(|v| v.to_str().ok()),
        Some(SUBJECT_ID)
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn brains_list_sends_both_auth_headers_from_env() {
    let server = mock_brains_list().await;
    let output = livingbrain(&server.uri())
        .env("LIVING_BRAIN_API_KEY", API_KEY)
        .env("LIVING_BRAIN_SUBJECT_ID", SUBJECT_ID)
        .args(["brains", "list"])
        .output()
        .expect("failed to run livingbrain");
    assert_success(output);
    assert_single_request_with_both_headers(&server).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn brains_list_sends_both_auth_headers_from_flags() {
    let server = mock_brains_list().await;
    let output = livingbrain(&server.uri())
        .args([
            "--api-key",
            API_KEY,
            "--subject-id",
            SUBJECT_ID,
            "brains",
            "list",
        ])
        .output()
        .expect("failed to run livingbrain");
    assert_success(output);
    assert_single_request_with_both_headers(&server).await;
}
