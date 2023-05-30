//! Sleppa release notification crate
//!
//! This crate aims at notify the team when a new release is publihed by sending a post to a communication platform.
//! This crate is generic over the platform with the trait [Notify] to implement.
//!
//! In order to be generic, this crate provides a trait to define the general behavior. The [Notify] trait is used to
//! publish a new post on the platform when a new release is published.
//!
//! Informations used to send the post are retrieved from a [Context] structure.
//! This context should contain a [NOTIFIER_KEY] associated with its [Configuration] structure.
//! This [configuration] should contain a [MESSAGE_KEY] to access the defined message to post.

mod constants;
mod errors;
pub mod mattermost;

use async_trait::async_trait;
use constants::{MESSAGE_KEY, NOTIFIER_KEY};
use errors::{NotifierError, NotifierResult};
use sleppa_configuration::{
    constants::{CONFIGURATION_KEY, CONFIGURATION_LAST_TAG},
    Context,
};

/// The plugin used to notify the new release
#[derive(Default)]
pub struct NotifierPlugin {
    /// The message to post for the new release
    message: String,
}

/// General behavior to post the message on the plaftorm
#[async_trait]
pub trait Notify {
    /// Sends the notification's post on the platform
    async fn notify_release(&self, context: &Context, message: String) -> NotifierResult<()>;
}

impl NotifierPlugin {
    /// Implementation of the NotifierPlugin::new() method
    pub fn new() -> Self {
        NotifierPlugin { message: String::new() }
    }

    /// Runs the plugin with an existing Context.
    ///
    /// The [Context] should contain a [Configuration] key.
    /// This [Configuration] should also contain the message fot the notification and possibly value needed by the
    /// used platform to post to.
    pub async fn run<T>(&mut self, context: &Context, instance: T) -> NotifierResult<()>
    where
        T: Notify,
    {
        let last_tag = match context.configurations[CONFIGURATION_KEY].map[CONFIGURATION_LAST_TAG].as_tag() {
            Some(value) => value,
            None => return Err(NotifierError::InvalidContext("No last tag found.".to_string())),
        };

        let message = match context.configurations[NOTIFIER_KEY].map[MESSAGE_KEY].as_string() {
            Some(value) => value,
            None => {
                return Err(NotifierError::InvalidContext(
                    "No message found for the notification".to_string(),
                ))
            }
        };

        // Sets the message by adding the new tag e.g. `New release (v3.2.1) !`
        self.message = format!("{} (v{}) !", message, last_tag.identifier);

        instance.notify_release(context, self.message.clone()).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
