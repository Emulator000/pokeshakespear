use std::str;

use actix_web::client::Client;

use serde::{Deserialize, Serialize};

pub async fn get<'a, S: AsRef<str>, T: ?Sized>(uri: S) -> Option<T>
where
    for<'de> T: Deserialize<'de> + 'a,
{
    let response = Client::default()
        .get(uri.as_ref())
        .header(super::USER_AGENT_KEY, super::UA)
        .send()
        .await;

    match response {
        Ok(mut response) => match response.body().await {
            Ok(body) => deserialize(body.to_vec()),
            _ => None,
        },
        _ => None,
    }
}

pub async fn post<'a, S: AsRef<str>, T: ?Sized, D: Serialize>(uri: S, request: D) -> Option<T>
where
    for<'de> T: Deserialize<'de> + 'a,
{
    let response = Client::default()
        .post(uri.as_ref())
        .header(super::USER_AGENT_KEY, super::UA)
        .send_json(&request)
        .await;

    match response {
        Ok(mut response) => match response.body().await {
            Ok(body) => deserialize(body.to_vec()),
            _ => None,
        },
        _ => None,
    }
}

pub fn deserialize<'a, T: ?Sized>(body: Vec<u8>) -> Option<T>
where
    for<'de> T: Deserialize<'de> + 'a,
{
    match serde_json::from_str(str::from_utf8(&body).unwrap_or("")) {
        Ok(result) => Some(result),
        _ => None,
    }
}
