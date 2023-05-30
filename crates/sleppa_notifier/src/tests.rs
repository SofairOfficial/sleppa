//! Unit tests
//!
//! This testing module implements the unit tests for testing `sleppa_notifier` crate routines.
//! To avoir a mocking of a server, we use crendentials as environment variable.
//!
//!//! Credentials to use are `log` and `pass`. The first defines the user id and the second its associated
//! password :
//! `pass="123abc456" log="mail@mail.com" cargo test -p sleppa_notifier`

use crate::{
    mattermost::{
        constants::{CHANNEL_ID_KEY, MATTERMOST_URL_KEY, TOKEN_KEY},
        Mattermost,
    },
    *,
};

use errors::TestResult;
use sleppa_configuration::Configuration;
use sleppa_primitives::{repositories::RepositoryTag, Value};
use std::collections::HashMap;

/// Tests that a message is correctly posted with a new [NotifierPlugin] instance.
#[tokio::test]
async fn test_can_run() -> TestResult<()> {
    // Unit test preparation
    // Retrieves the user id and password in the environment variable.
    let password = std::env::var("pass").unwrap();
    let login = std::env::var("log").unwrap();

    // Constructs a Mattermost instance to retrieve a session token
    let mut mm = Mattermost::new("https://sofairofficial.cloud.mattermost.com", None);
    mm.login(login, password).await?;
    let token = mm.authentication_token.unwrap();

    // Constructs a new tag
    let new_tag = RepositoryTag {
        identifier: "3.2.1".to_string(),
        hash: "123abc456def".to_string(),
    };

    let mut notifier = NotifierPlugin::new();

    let mut context = Context {
        configurations: HashMap::new(), //HashMap<String, Configuration>
    };

    // Creates a [Configuration] for the notifier plugin
    let mut config = Configuration {
        map: HashMap::new(), //HashMap<String, Value>
    };
    // Creates a [Configuration] for the general configuration plugin
    let mut general_config = Configuration {
        map: HashMap::new(), //HashMap<String, Value>
    };

    // Populates the Configuration
    config
        .map
        .insert(MESSAGE_KEY.to_string(), Value::String("New release".to_string()));

    config.map.insert(
        CHANNEL_ID_KEY.to_string(),
        Value::String("p1kfdiyg53gzjki1cpsfr4fzwe".to_string()),
    );

    config.map.insert(
        MATTERMOST_URL_KEY.to_string(),
        Value::String("https://sofairofficial.cloud.mattermost.com".to_string()),
    );

    config.map.insert(TOKEN_KEY.to_string(), Value::String(token));

    // Populates the Context
    context.configurations.insert(NOTIFIER_KEY.to_string(), config);

    // Populates the general Configuration
    general_config
        .map
        .insert(CONFIGURATION_LAST_TAG.to_string(), Value::Tag(new_tag));

    // Populates the Context
    context
        .configurations
        .insert(CONFIGURATION_KEY.to_string(), general_config);

    // Creates the plugin
    let mattermost = Mattermost::new(
        context.configurations[&NOTIFIER_KEY.to_string()].map[&MATTERMOST_URL_KEY.to_string()]
            .as_string()
            .unwrap(),
        Some(
            context.configurations[&NOTIFIER_KEY.to_string()].map[&TOKEN_KEY.to_string()]
                .as_string()
                .unwrap()
                .to_string(),
        ),
    );

    // Asserts the message is correctly published by the plugin
    assert!(notifier.run(&context, mattermost).await.is_ok());

    Ok(())
}
