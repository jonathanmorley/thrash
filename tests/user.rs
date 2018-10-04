extern crate failure;
extern crate mockito;
extern crate thrash;

use failure::Error;
use mockito::{mock, Matcher};

#[test]
fn users_admin() -> Result<(), Error> {
    let _m = mock(
        "GET",
        Matcher::Regex(r"^/rest/api/1.0/admin/users(\?.+)?$".to_string()),
    ).with_status(200)
    .with_header("content-type", "application/json")
    .with_body(format!(
        r#"{{
            "size": 2,
            "limit": 25,
            "isLastPage": true,
            "values": [
                {{
                    "name": "foo-user",
                    "emailAddress": "foo-user@example.com",
                    "id": 20,
                    "displayName": "Foo User",
                    "active": true,
                    "slug": "foouser",
                    "type": "NORMAL",
                    "directoryName": "Stash Internal Directory",
                    "deletable": true,
                    "mutableDetails": true,
                    "mutableGroups": true,
                    "links": {{
                        "self": [
                            {{
                                "href": "https://{url}/users/foouser"
                            }}
                        ]
                    }}
                }},
                {{
                    "name": "bar-user",
                    "emailAddress": "bar-user@example.com",
                    "id": 20,
                    "displayName": "Bar User",
                    "active": true,
                    "slug": "baruser",
                    "type": "NORMAL",
                    "directoryName": "Stash Internal Directory",
                    "deletable": true,
                    "mutableDetails": true,
                    "mutableGroups": true,
                    "links": {{
                        "self": [
                            {{
                                "href": "https://{url}/users/baruser"
                            }}
                        ]
                    }}
                }}
            ],
            "start": 0
        }}"#,
        url = mockito::SERVER_URL
    )).create();

    let mut client = thrash::client::Client::new(mockito::SERVER_URL, "user", "password")?;

    assert_eq!(client.users_admin()?.len(), 2);

    Ok(())
}

#[test]
fn users() -> Result<(), Error> {
    let _m = mock(
        "GET",
        Matcher::Regex(r"^/rest/api/1.0/users(\?.+)?$".to_string()),
    ).with_status(200)
    .with_header("content-type", "application/json")
    .with_body(format!(
        r#"{{
            "size": 2,
            "limit": 25,
            "isLastPage": true,
            "values": [
                {{
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
                {{
                    "name": "bar-user",
                    "emailAddress": "bar-user@example.com",
                    "id": 20,
                    "displayName": "Bar User",
                    "active": true,
                    "slug": "baruser",
                    "type": "NORMAL",
                    "links": {{
                        "self": [
                            {{
                                "href": "https://{url}/users/baruser"
                            }}
                        ]
                    }}
                }}
            ],
            "start": 0
        }}"#,
        url = mockito::SERVER_URL
    )).create();

    let mut client = thrash::client::Client::new(mockito::SERVER_URL, "user", "password")?;

    assert_eq!(client.users()?.len(), 2);

    Ok(())
}

#[test]
fn user() -> Result<(), Error> {
    let _m = mock(
        "GET",
        Matcher::Regex(r"^/rest/api/1.0/users/foouser(\?.+)?$".to_string()),
    ).with_status(200)
    .with_header("content-type", "application/json")
    .with_body(format!(
        r#"{{
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
        }}"#,
        url = mockito::SERVER_URL
    )).create();

    let mut client = thrash::client::Client::new(mockito::SERVER_URL, "user", "password")?;

    assert_eq!(client.user("foouser")?.name(), "foo-user");

    Ok(())
}
