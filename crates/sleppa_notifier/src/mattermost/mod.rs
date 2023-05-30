//! [Mattermost ](https://mattermost.com/) notification module
//!
//! This module implements the notification of a new release on the Mattermost platform.
//! It uses the Mattermost's [API](https://api.mattermost.com/) to defines the [CreatePost] structure
//! in order to serialize it and send the json request to the server.
//!
//! It implements the trait [crate::Notify] to provide a way to send the post to the platform.
//!
//! A [Mattermost] instance defines a way to send requests to the Mattermost's API using a token. The token is
//! usually a [personnal authentication token](https://docs.mattermost.com/developer/personal-access-tokens.html).
//! The post is send to a channel on Mattermost where the user must have the `create_post` permission.
//! This channel is defined by its ID which can be retrieved using a Mattermost `GET` request at the url :
//! `https://your-mattermost-url.com/api/v4/teams/{team_id}/channels/name/{channel_name}`.
//!
//! The `team_id` can be retrieved using a Mattermost `GET` request at the url :
//! `https://your-mattermost-url.com/api/v4/teams`
//!
//! More information on the Mattermost's API documention :
//!  - [channel id](https://api.mattermost.com/#tag/channels/operation/GetChannelByName)
//!  - [team GUID](https://api.mattermost.com/#tag/teams/operation/GetAllTeams)
//!
//! Informations used to send the post to a Mattermost instance are retrived from a [Context] structure.
//! This context should contain a [NOTIFIER_KEY] associated with its [Configuration] structure.
//! This [configuration] should contain :
//!  - [MATTERMOST_URL_KEY] to access the url of the instance
//!  - [CHANNEL_ID_KEY] to access the channel id to post to
//!  - [TOKEN_KEY] to access the token used as credential.

pub mod constants;
pub mod errors;

use crate::{
    constants::NOTIFIER_KEY,
    errors::{NotifierError, NotifierResult},
    mattermost::{
        constants::{CHANNEL_ID_KEY, MATTERMOST_URL_KEY, TOKEN_KEY},
        errors::{MattermostError, MattermostResult},
    },
    Notify,
};
use async_trait::async_trait;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sleppa_configuration::Context;

/// Defines the CreatePost object defined by Mattermost's API
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePost {
    /// The channel to post in
    channel_id: String,
    /// The message to post
    message: String,
}

/// Defines a Mattermost instance.
///
/// This structure represents a Mattermost instance to communicate with.
/// It is defined by the url of the instance (e.g. https://somemattermostinstance.com), the authentication
/// to login into the instance, an HTTP client and the token to authenticate to.
/// The authentication_token is required. A session token should be retrieved if the authentication is made by
/// an user login. The [login] method does this process.
pub struct Mattermost {
    /// The url of the Mattermost instance to post to.
    pub(crate) instance_url: String,
    /// The http client using [reqwest].
    pub(crate) client: Client,
    /// The token needed to authenticate. Usually a personnale acces token.
    pub(crate) authentication_token: Option<String>,
}

impl CreatePost {
    /// Builds a CreatePost with a given channel and message.
    pub fn build(channel: &str, message: &str) -> Self {
        CreatePost {
            channel_id: channel.to_string(),
            message: message.to_string(),
        }
    }
}

impl Mattermost {
    /// Creates a new Mattermost struct with a given instance's url and optionnal token.
    pub fn new(url: &str, token: Option<String>) -> Self {
        Self {
            instance_url: url.to_string(),
            client: Client::new(),
            authentication_token: token,
        }
    }

    /// Gets the token from a user's login and password.
    ///
    /// This method is only usefull when a [Mattermost] is instantiate without a personnel access token.
    /// This retrieve a session token from a user's login.
    pub async fn login(&mut self, user: String, password: String) -> MattermostResult<()> {
        // If the token is already defined then the method stops.
        if self.authentication_token.is_some() {
            return Ok(());
        }
        let url = format!("{}/api/v4/users/login", self.instance_url);

        // Sends the user's login to the Mattermost instance.
        let response = self
            .client
            .post(&url)
            .json(&json!({
                "login_id": user,
                "password": password,
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(MattermostError::RequestError(response.status().to_string()));
        }

        // Retrieves the session token from the response.
        let session_token = match response.headers().get("Token") {
            Some(value) => value,
            None => return Err(MattermostError::ErrorToken()),
        };

        // Sets the token to be used by futur login process.
        self.authentication_token = Some(session_token.to_str()?.to_string());

        Ok(())
    }

    /// Posts a new message on a Mattermost intance.
    ///
    /// This method posts a given [Post] on the specified Mattermost instance.
    /// To do so, the [Mattermost] structure needs a token to authenticate.
    pub async fn post(&self, post: CreatePost) -> MattermostResult<()> {
        let url = format!("{}/api/v4/posts", self.instance_url);

        let token = match &self.authentication_token {
            Some(value) => format!("Bearer {}", value),
            None => return Err(MattermostError::ErrorToken()),
        };

        // Constructs the header of the request.
        let mut map = HeaderMap::new();
        map.insert(header::ACCEPT, HeaderValue::from_static("application/json"));
        map.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
        map.insert(header::AUTHORIZATION, HeaderValue::from_str(token.as_str())?);

        // Constructs the body of the request.
        let body = json!({
            "channel_id": post.channel_id,
            "message": post.message,
        });

        let response = self.client.post(url).headers(map).json(&body).send().await?;

        if !response.status().is_success() {
            return Err(MattermostError::RequestError(response.status().to_string()));
        }

        Ok(())
    }
}

#[async_trait]
impl Notify for Mattermost {
    /// Notifies the new release on a Mattermost instance.
    ///
    /// Implementation of the trait [Notify] to send a new post when a new release is published.
    /// The [Post] is converted to a [CreatePost] in order to be serialized in json to send the request
    /// to the Mattermost's API.
    async fn notify_release(&self, context: &Context, message: String) -> NotifierResult<()> {
        // Retrieves the value from the [Context].
        let channel_id = match context.configurations[NOTIFIER_KEY].map[CHANNEL_ID_KEY].as_string() {
            Some(value) => value,
            None => return Err(NotifierError::InvalidContext("No channel ID found.".to_string())),
        };

        let token = match context.configurations[NOTIFIER_KEY].map[TOKEN_KEY].as_string() {
            Some(value) => value,
            None => {
                return Err(NotifierError::InvalidContext(
                    "No token found for authentication.".to_string(),
                ))
            }
        };

        let mattermost_url = match context.configurations[NOTIFIER_KEY].map[MATTERMOST_URL_KEY].as_string() {
            Some(value) => value,
            None => {
                return Err(NotifierError::InvalidContext(
                    "No URL found for Mattermost instance.".to_string(),
                ))
            }
        };

        let post_to_send = CreatePost::build(channel_id, message.as_str());

        let mattermost = Mattermost::new(mattermost_url, Some(token.to_string()));

        // Publishes a new post on Mattermost
        match mattermost.post(post_to_send).await {
            Ok(()) => Ok(()),
            Err(err) => {
                return Err(NotifierError::SendingError(err.to_string()));
            }
        }
    }
}

#[cfg(test)]
mod tests;
