extern crate failure;
extern crate mockito;
extern crate thrash;

use failure::Error;
use mockito::{mock, Matcher};

#[test]
fn projects() -> Result<(), Error> {
    let _m = mock(
        "GET",
        Matcher::Regex(r"^/rest/api/1.0/projects(\?.+)?$".to_string()),
    ).with_status(200)
    .with_header("content-type", "application/json")
    .with_body(format!(
        r#"{{
            "size": 2,
            "limit": 25,
            "isLastPage": true,
            "values": [
                {{
                    "key": "FOO",
                    "id": 1,
                    "name": "Foo Project",
                    "description": "Foo Project Description",
                    "public": false,
                    "type": "NORMAL",
                    "links": {{
                        "self": [
                            {{
                                "href": "{url}/projects/FOO"
                            }}
                        ]
                    }}
                }},
                {{
                    "key": "BAR",
                    "id": 2,
                    "name": "Bar Project",
                    "description": "Bar Project Description",
                    "public": false,
                    "type": "NORMAL",
                    "links": {{
                        "self": [
                            {{
                                "href": "{url}/projects/ADR"
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

    assert_eq!(client.projects()?.len(), 2);

    Ok(())
}

#[test]
fn project() -> Result<(), Error> {
    let _m = mock(
        "GET",
        Matcher::Regex(r"^/rest/api/1.0/projects/FOO(\?.+)?$".to_string()),
    ).with_status(200)
    .with_header("content-type", "application/json")
    .with_body(format!(
        r#"{{
            "key": "FOO",
            "id": 1,
            "name": "Foo Project",
            "description": "Foo Project Description",
            "public": false,
            "type": "NORMAL",
            "links": {{
                "self": [
                    {{
                        "href": "{url}/projects/FOO"
                    }}
                ]
            }}
        }}"#,
        url = mockito::SERVER_URL
    )).create();

    let mut client = thrash::client::Client::new(mockito::SERVER_URL, "user", "password")?;

    assert_eq!(client.project("FOO")?.name(), "Foo Project");

    Ok(())
}
