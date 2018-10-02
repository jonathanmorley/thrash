extern crate bitbucket;
extern crate failure;
extern crate mockito;

use failure::Error;
use mockito::{mock, Matcher};
use std::path::PathBuf;

#[test]
fn repositories() -> Result<(), Error> {
    let _m = mock(
        "GET",
        Matcher::Regex(r"^/rest/api/1.0/projects/FOO/repos(\?.+)?$".to_string()),
    ).with_status(200)
    .with_header("content-type", "application/json")
    .with_body(format!(
        r#"{{
            "size": 2,
            "limit": 25,
            "isLastPage": true,
            "values": [
                {{
                    "slug": "foo-repo-1",
                    "id": 10,
                    "name": "foo-repo-1",
                    "scmId": "git",
                    "state": "AVAILABLE",
                    "statusMessage": "Available",
                    "forkable": true,
                    "project": {{
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
                    "public": false,
                    "links": {{
                        "clone": [
                            {{
                                "href": "ssh://git@{url}/foo/foo-repo-1.git",
                                "name": "ssh"
                            }},
                            {{
                                "href": "https://test@{url}/scm/foo/foo-repo-1.git",
                                "name": "http"
                            }}
                        ],
                        "self": [
                            {{
                                "href": "https://{url}/projects/FOO/repos/foo-repo-1/browse"
                            }}
                        ]
                    }}
                }},
                {{
                    "slug": "foo-repo-2",
                    "id": 10,
                    "name": "foo-repo-2",
                    "scmId": "git",
                    "state": "AVAILABLE",
                    "statusMessage": "Available",
                    "forkable": true,
                    "project": {{
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
                    "public": false,
                    "links": {{
                        "clone": [
                            {{
                                "href": "ssh://git@{url}/foo/foo-repo-2.git",
                                "name": "ssh"
                            }},
                            {{
                                "href": "https://test@{url}/scm/foo/foo-repo-2.git",
                                "name": "http"
                            }}
                        ],
                        "self": [
                            {{
                                "href": "https://{url}/projects/FOO/repos/foo-repo-2/browse"
                            }}
                        ]
                    }}
                }}
            ],
            "start": 0
        }}"#,
        url = mockito::SERVER_URL
    )).create();

    let mut client = bitbucket::client::Client::new(mockito::SERVER_URL, "user", "password")?;

    assert_eq!(client.repositories("FOO")?.len(), 2);

    Ok(())
}

#[test]
fn repository() -> Result<(), Error> {
    let _m = mock(
        "GET",
        Matcher::Regex(r"^/rest/api/1.0/projects/FOO/repos/foo-repo-1(\?.+)?$".to_string()),
    ).with_status(200)
    .with_header("content-type", "application/json")
    .with_body(format!(
        r#"{{
            "slug": "foo-repo-1",
            "id": 10,
            "name": "foo-repo-1",
            "scmId": "git",
            "state": "AVAILABLE",
            "statusMessage": "Available",
            "forkable": true,
            "project": {{
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
            "public": false,
            "links": {{
                "clone": [
                    {{
                        "href": "ssh://git@{url}/foo/foo-repo-1.git",
                        "name": "ssh"
                    }},
                    {{
                        "href": "https://test@{url}/scm/foo/foo-repo-1.git",
                        "name": "http"
                    }}
                ],
                "self": [
                    {{
                        "href": "https://{url}/projects/FOO/repos/foo-repo-1/browse"
                    }}
                ]
            }}
        }}"#,
        url = mockito::SERVER_URL
    )).create();

    let mut client = bitbucket::client::Client::new(mockito::SERVER_URL, "user", "password")?;

    assert_eq!(client.repository("FOO", "foo-repo-1")?.name(), "foo-repo-1");

    Ok(())
}

#[test]
fn repository_files() -> Result<(), Error> {
    let _m = mock(
        "GET",
        Matcher::Regex(r"^/rest/api/1.0/projects/FOO/repos/foo-repo-1/files(\?.+)?$".to_string()),
    ).with_status(200)
    .with_header("content-type", "application/json")
    .with_body(
        r#"{
            "size": 2,
            "limit": 25,
            "isLastPage": true,
            "values": [
                "foo",
                "bar"
            ],
            "start": 0
        }"#,
    ).create();

    let mut client = bitbucket::client::Client::new(mockito::SERVER_URL, "user", "password")?;

    assert_eq!(
        client.repository_files("FOO", "foo-repo-1")?,
        vec![PathBuf::from("foo"), PathBuf::from("bar")]
    );

    Ok(())
}

#[test]
fn repository_file_contents() -> Result<(), Error> {
    let _m = mock(
        "GET",
        Matcher::Regex(
            r"^/rest/api/1.0/projects/FOO/repos/foo-repo-1/browse/foo(\?.+)?$".to_string(),
        ),
    ).with_status(200)
    .with_header("content-type", "application/json")
    .with_body(
        r#"{
            "lines": [
                {
                    "text": "line1"
                },
                {
                    "text": "line2"
                }
            ],
            "start": 0,
            "size": 2,
            "isLastPage": true
        }"#,
    ).create();

    let mut client = bitbucket::client::Client::new(mockito::SERVER_URL, "user", "password")?;

    assert_eq!(
        client.repository_file_contents("FOO", "foo-repo-1", "foo")?,
        "line1\nline2"
    );

    Ok(())
}
