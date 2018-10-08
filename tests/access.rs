extern crate failure;
extern crate mockito;
extern crate thrash;

use failure::Error;
use mockito::{mock, Matcher};

#[test]
fn group_access() -> Result<(), Error> {
    let _m = mock(
        "GET",
        Matcher::Regex(r"^/rest/api/1.0/admin/permissions/groups(\?.+)?$".to_string()),
    ).with_status(200)
    .with_header("content-type", "application/json")
    .with_body(
        r#"{
            "size": 2,
            "limit": 25,
            "isLastPage": true,
            "values": [
                {
                    "group": {
                        "name": "foo-group"
                    },
                    "permission": "ADMIN"
                },
                {
                    "group": {
                        "name": "bar-group"
                    },
                    "permission": "READ"
                }
            ],
            "start": 0
        }"#,
    ).create();

    let client = thrash::client::Client::new(mockito::SERVER_URL, "user", "password")?;

    assert_eq!(client.group_access()?.len(), 2);

    Ok(())
}

#[test]
fn project_group_access() -> Result<(), Error> {
    let _m = mock(
        "GET",
        Matcher::Regex(r"^/rest/api/1.0/projects/FOO/permissions/groups(\?.+)?$".to_string()),
    ).with_status(200)
    .with_header("content-type", "application/json")
    .with_body(
        r#"{
            "size": 2,
            "limit": 25,
            "isLastPage": true,
            "values": [
                {
                    "group": {
                        "name": "foo-group"
                    },
                    "permission": "PROJECT_ADMIN"
                },
                {
                    "group": {
                        "name": "bar-group"
                    },
                    "permission": "PROJECT_READ"
                }
            ],
            "start": 0
        }"#,
    ).create();

    let client = thrash::client::Client::new(mockito::SERVER_URL, "user", "password")?;

    assert_eq!(client.project_group_access("FOO")?.len(), 2);

    Ok(())
}

#[test]
fn repository_group_access() -> Result<(), Error> {
    let _m = mock(
        "GET",
        Matcher::Regex(
            r"^/rest/api/1.0/projects/FOO/repos/foo-repo-1/permissions/groups(\?.+)?$".to_string(),
        ),
    ).with_status(200)
    .with_header("content-type", "application/json")
    .with_body(
        r#"{
            "size": 2,
            "limit": 25,
            "isLastPage": true,
            "values": [
                {
                    "group": {
                        "name": "foo-group"
                    },
                    "permission": "REPO_ADMIN"
                },
                {
                    "group": {
                        "name": "bar-group"
                    },
                    "permission": "REPO_READ"
                }
            ],
            "start": 0
        }"#,
    ).create();

    let client = thrash::client::Client::new(mockito::SERVER_URL, "user", "password")?;

    assert_eq!(
        client.repository_group_access("FOO", "foo-repo-1")?.len(),
        2
    );

    Ok(())
}

#[test]
fn project_user_access() -> Result<(), Error> {
    let _m = mock(
        "GET",
        Matcher::Regex(r"^/rest/api/1.0/projects/FOO/permissions/users(\?.+)?$".to_string()),
    ).with_status(200)
    .with_header("content-type", "application/json")
    .with_body(format!(
        r#"{{
            "size": 1,
            "limit": 25,
            "isLastPage": true,
            "values": [
                {{
                    "user": {{
                        "name": "foo-user",
                        "emailAddress": "foo-user@example.com",
                        "id": 20,
                        "displayName": "Foo User",
                        "active": true,
                        "slug": "foouser",
                        "type": "NORMAL",
                        "links": {{
                            "self": [
                                {{
                                    "href": "https://{url}/users/foouser"
                                }}
                            ]
                        }}
                    }},
                    "permission": "PROJECT_ADMIN"
                }}
            ],
            "start": 0
        }}"#,
        url = mockito::SERVER_URL
    )).create();

    let client = thrash::client::Client::new(mockito::SERVER_URL, "user", "password")?;

    assert_eq!(client.project_user_access("FOO")?.len(), 1);

    Ok(())
}

#[test]
fn repository_user_access() -> Result<(), Error> {
    let _m = mock(
        "GET",
        Matcher::Regex(
            r"^/rest/api/1.0/projects/FOO/repos/foo-repo-1/permissions/users(\?.+)?$".to_string(),
        ),
    ).with_status(200)
    .with_header("content-type", "application/json")
    .with_body(format!(
        r#"{{
            "size": 1,
            "limit": 25,
            "isLastPage": true,
            "values": [
                {{
                    "user": {{
                        "name": "foo-user",
                        "emailAddress": "foo-user@example.com",
                        "id": 20,
                        "displayName": "Foo User",
                        "active": true,
                        "slug": "foouser",
                        "type": "NORMAL",
                        "links": {{
                            "self": [
                                {{
                                    "href": "https://{url}/users/foouser"
                                }}
                            ]
                        }}
                    }},
                    "permission": "REPO_ADMIN"
                }}
            ],
            "start": 0
        }}"#,
        url = mockito::SERVER_URL
    )).create();

    let client = thrash::client::Client::new(mockito::SERVER_URL, "user", "password")?;

    assert_eq!(client.repository_user_access("FOO", "foo-repo-1")?.len(), 1);

    Ok(())
}
