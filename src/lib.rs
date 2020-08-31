use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use std::*;

use tracing::{error, trace};

#[derive(Default, Clone, Serialize, Debug)]
#[serde(default)]
pub struct PushoverMessage {
    pub token: String,
    pub user: String,
    pub message: String,
    pub devices: Vec<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub url_title: Option<String>,
    pub priority: Option<i8>,
    pub timestamp: Option<String>,
    pub html: Option<u8>,
    pub retry: Option<u8>,
    pub expire: Option<u8>,
}

impl PushoverMessage {
    pub fn covid_monitor_message() -> PushoverMessage {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        PushoverMessage {
            // token: "ay7q9t4gddfqhni76e7q6c163izorz".to_string(),
            user: "uwwcfgvbq3acat2c3quwz5i3v1ik2c".to_string(),
            timestamp: Some(format!("{}", since_the_epoch.as_millis())),
            html: Some(1),
            ..Default::default()
        }
    }
    pub fn normal_msg(message: &str) -> PushoverMessage {
        PushoverMessage {
            message: message.to_string(),
            ..PushoverMessage::covid_monitor_message()
        }
    }
    pub fn priority_msg(message: &str) -> PushoverMessage {
        PushoverMessage {
            priority: Some(2),
            retry: Some(30),
            expire: Some(120),
            ..PushoverMessage::normal_msg(message)
        }
    }
}

#[derive(Debug)]
struct PushoverError {
    details: String,
}

impl PushoverError {
    fn new(msg: &str) -> PushoverError {
        PushoverError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for PushoverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for PushoverError {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Token is associate with Pushover Application
pub struct Pushover {
    token: String,
    //client: std::sync::Mutex<awc::Client>,
}

impl Pushover {
    const URL: &'static str = "http://api.pushover.net/1/messages.json";

    pub fn new_with_token<T: Into<String>>(token: T) -> Self {
        Pushover {
            token: token.into(),
        }
    }

    pub async fn send_message(
        &self,
        message: PushoverMessage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        // let mut msg = message.clone();
        let mut message = message;
        message.token = self.token.clone();

        let response = client.post(Pushover::URL).json(&message).send().await;
        match response {
            Ok(response) => {
                if response.status() != reqwest::StatusCode::OK {
                    error!("Not possible to send message");
                    return Err(Box::new(PushoverError::new("Status code not 200")));
                //return Err("Pushover server status not 200");
                } else {
                    trace!("Notification send to pushoverapp");
                }
            }
            Err(err) => {
                error!("Not possible to send message");
                return Err(Box::new(err));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn send_message_error() {
        let url = "http://api.pushover.net/1/messages.json";
        // let _push = Pushover::default();
        assert_eq!(&Pushover::URL, &url);
    }
}
