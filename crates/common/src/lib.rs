use std::fmt::Debug;

use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};

pub trait Answer {
    fn code(&self) -> i32;
    fn process_error(&self);
    fn process(&mut self);
}

pub(crate) async fn call<U: Serialize + Debug, V: DeserializeOwned + Answer>(
    url: String,
    data: Option<&U>,
    first_route: &str,
    second_route: &str,
) {
    let client = match data {
        Some(js) => Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
            .post(format!("{}/{}/{}", url, first_route, second_route))
            .json(js)
            .send(),
        None => Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
            .get(format!("{}/{}/{}", url, first_route, second_route))
            .send(),
    };
    match client.await {
        Ok(e) => match e.json::<V>().await {
            Ok(mut answer) => {
                if answer.code() != 200 {
                    answer.process_error();
                } else {
                    answer.process();
                }
            }
            Err(e) => {
                println!("Error while deserializing answer: {e}");
            }
        },
        Err(e) => {
            println!("Error while sending the resquest: {e}");
        }
    }
}
