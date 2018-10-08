use failure::Error;
use percent_encoding::{utf8_percent_encode, SIMPLE_ENCODE_SET};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use url::Url;

pub struct Client {
    client: reqwest::Client,
    base_url: String,
    auth: Authentication,
}

pub struct Authentication {
    username: String,
    password: String,
}

impl Client {
    pub fn new(base_url: &str, username: &str, password: &str) -> Result<Client, Error> {
        Ok(Client {
            client: reqwest::Client::builder()
                .danger_accept_invalid_hostnames(true)
                .build()?,
            base_url: base_url.to_owned(),
            auth: Authentication {
                username: username.to_owned(),
                password: password.to_owned(),
            },
        })
    }

    pub fn get<T>(&self, path: &str) -> Result<T, Error>
    where
        for<'de> T: Deserialize<'de>,
    {
        let url = Url::parse(&format!("{}/{}", self.base_url, path))?;

        info!("GET {}", url);
        Ok(self
            .client
            .get(url.clone())
            .basic_auth(self.auth.username.clone(), Some(self.auth.password.clone()))
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn get_paged<T>(&self, path: &str) -> Result<Vec<T>, Error>
    where
        for<'de> T: Deserialize<'de>,
    {
        let mut output = Vec::new();
        let mut start = 0;

        loop {
            let mut page: Page<T> = self.get(&format!("{}?limit=1000&start={}", path, start))?;
            output.append(&mut page.values);

            if page.is_last_page {
                break;
            } else {
                start += page.size
            }
        }

        Ok(output)
    }

    pub fn get_lines_paged(&self, path: &str) -> Result<Vec<String>, Error> {
        let mut output = Vec::new();
        let mut start = 0;

        loop {
            let page: LinePage = self.get(&format!("{}?limit=1000&start={}", path, start))?;
            let mut text = page.lines.into_iter().map(|l| l.text).collect();
            output.append(&mut text);

            if page.is_last_page {
                break;
            } else {
                start += page.size
            }
        }

        Ok(output)
    }

    pub fn put<T>(&self, path: &str, value: Option<&T>) -> Result<(), Error>
    where
        T: Serialize,
    {
        let url = Url::parse(&format!("{}{}", self.base_url, path))?;

        info!("PUT {} {}", url, serde_json::to_string(&value)?);
        let req = self
            .client
            .put(url)
            .basic_auth(self.auth.username.clone(), Some(self.auth.password.clone()));

        if let Some(body) = value {
            req.json(&body).send()?.error_for_status()?;
        } else {
            req.send()?.error_for_status()?;
        }

        Ok(())
    }

    pub fn delete(&self, path: &str) -> Result<(), Error> {
        let url = Url::parse(&format!("{}{}", self.base_url, path))?;

        info!("DELETE {}", url);
        self.client
            .delete(url)
            .basic_auth(self.auth.username.clone(), Some(self.auth.password.clone()))
            .send()?
            .error_for_status()?;

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Page<T> {
    size: u64,
    limit: u64,
    is_last_page: bool,
    values: Vec<T>,
    start: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LinePage {
    lines: Vec<Line>,
    start: u64,
    size: u64,
    is_last_page: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Line {
    text: String,
}

define_encode_set! {
    pub QUERY_ENCODE_SET = [SIMPLE_ENCODE_SET] | {'&'}
}

pub fn percent_encode(s: &str) -> String {
    utf8_percent_encode(s, QUERY_ENCODE_SET).collect::<String>()
}
