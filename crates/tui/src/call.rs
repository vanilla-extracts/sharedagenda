use std::fmt::{Debug, Display};

use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};

pub trait Answer {
    fn code(&self) -> i32;
    fn process_error(&self);
    fn process(&mut self);
}

#[derive(Debug, Clone)]
pub struct CallError {
    reason: String,
}

impl Display for CallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl std::error::Error for CallError {
    fn description(&self) -> &str {
        self.reason.as_str()
    }
}

pub async fn call<U: Serialize + Debug, V: DeserializeOwned + Answer>(
    url: String,
    data: Option<&U>,
    first_route: &str,
    second_route: &str,
) -> Result<(), CallError> {
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
                    Ok(())
                } else {
                    answer.process();
                    Ok(())
                }
            }
            Err(e) => Err(CallError {
                reason: format!("Error while deserializing answer: {e}"),
            }),
        },
        Err(e) => Err(CallError {
            reason: format!("Error while sending the resquest: {e}"),
        }),
    }
}
