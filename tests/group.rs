extern crate failure;
extern crate mockito;
extern crate thrash;

use failure::Error;
use mockito::{mock, Matcher};

#[test]
fn groups_admin() -> Result<(), Error> {
    let _m = mock(
        "GET",
        Matcher::Regex(r"^/rest/api/1.0/admin/groups(\?.+)?$".to_string()),
    ).with_status(200)
    .with_header("content-type", "application/json")
    .with_body(
        r#"{
            "size": 2,
            "limit": 25,
            "isLastPage": true,
            "values": [
                {
                    "name": "foo-group",
                    "deletable": true
                },
                {
                    "name": "bar-group",
                    "deletable": true
                }
            ],
            "start": 0
        }"#,
    ).create();

    let mut client = thrash::client::Client::new(mockito::SERVER_URL, "user", "password")?;

    assert_eq!(client.groups_admin()?.len(), 2);

    Ok(())
}

#[test]
fn groups() -> Result<(), Error> {
    let _m = mock(
        "GET",
        Matcher::Regex(r"^/rest/api/1.0/groups(\?.+)?$".to_string()),
    ).with_status(200)
    .with_header("content-type", "application/json")
    .with_body(
        r#"{
            "size": 2,
            "limit": 25,
            "isLastPage": true,
            "values": [
                "foo-group",
                "bar-group"
            ],
            "start": 0
        }"#,
    ).create();

    let mut client = thrash::client::Client::new(mockito::SERVER_URL, "user", "password")?;

    assert_eq!(client.groups()?.len(), 2);

    Ok(())
}
