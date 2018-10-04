extern crate failure;
extern crate mockito;
extern crate thrash;

use failure::Error;
use mockito::{mock, Matcher};

#[test]
fn project_default_permission_none() -> Result<(), Error> {
    let _read = mock(
        "GET",
        Matcher::Regex(
            r"^/rest/api/1.0/projects/FOO/permissions/PROJECT_READ/all(\?.+)?$".to_string(),
        ),
    ).with_status(200)
    .with_header("content-type", "application/json")
    .with_body(r#"{ "permitted": false }"#)
    .create();

    let _write = mock(
        "GET",
        Matcher::Regex(
            r"^/rest/api/1.0/projects/FOO/permissions/PROJECT_WRITE/all(\?.+)?$".to_string(),
        ),
    ).with_status(200)
    .with_header("content-type", "application/json")
    .with_body(r#"{ "permitted": false }"#)
    .create();

    let _admin = mock(
        "GET",
        Matcher::Regex(
            r"^/rest/api/1.0/projects/FOO/permissions/PROJECT_ADMIN/all(\?.+)?$".to_string(),
        ),
    ).with_status(200)
    .with_header("content-type", "application/json")
    .with_body(r#"{ "permitted": false }"#)
    .create();

    let mut client = thrash::client::Client::new(mockito::SERVER_URL, "user", "password")?;

    assert_eq!(client.project_default_permission("FOO")?, "PROJECT_NONE");

    Ok(())
}
