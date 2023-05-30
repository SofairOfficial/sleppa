//! Unit tests
//!
//! This testing module implements the unit tests for testing mattermost module routines.
//! To avoir a mocking of a server, we use crendentials as environment variable.
//! Credentials to use are `log` and `pass`. The first defines the user id and the second its associated
//! password :
//! `pass="123abc456" log="mail@mail.com" cargo test -p sleppa_notifier`

use crate::mattermost::{errors::*, *};

// Builds a correct CreatedPost instance with a given channel_id and message
#[test]
fn test_can_build() {
    // Unit test preparation
    let channel = "channel identifier";
    let message = "Some message to post";

    // Execution step
    let created_post = CreatePost::build(channel, message);

    // Asserts the result is correct
    assert_eq!(created_post.channel_id, channel);
    assert_eq!(created_post.message, message);
}

// Builds a correct Mattermost instance with a given user crendentials
#[test]
fn test_can_new() {
    // Unit test preparation
    let token = "123abc456def".to_string();
    let url = "www.mattermost.com";

    // Execution step
    let mattermost = Mattermost::new(url, Some(token.clone()));

    // Asserts the instance is correct
    assert_eq!(mattermost.instance_url, url);
    assert_eq!(mattermost.authentication_token, Some(token));
}

// Tests that a login is correctly done with good datas to a Mattermost instance
#[tokio::test]
async fn test_can_login() -> TestResult<()> {
    // Unit test preparation
    // Retrieves the user id and password in the environment variable.
    let password = std::env::var("pass").unwrap();
    let login = std::env::var("log").unwrap();
    let url = "https://sofairofficial.cloud.mattermost.com";

    // Constructs a good authentication data
    let good_authdata = (login.clone(), password.clone());
    // Constructs a wrong authentication data wiht wrong password
    let wrong_authdata = (login, "wrong_password".to_string());

    // Constructs a wrong authentication data wiht wrong login_id
    let wronglogin_authdata = ("wrong@mail.com".to_string(), password);

    // Asserts the token is correctly retrieved
    assert!(Mattermost::new(url, None)
        .login(good_authdata.0.clone(), good_authdata.1.clone())
        .await
        .is_ok());
    // Asserts an error occured with a wrong password
    assert!(Mattermost::new(url, None)
        .login(wrong_authdata.0, wrong_authdata.1)
        .await
        .is_err());
    // Asserts an error occured with a wrong login_id
    assert!(Mattermost::new(url, None)
        .login(wronglogin_authdata.0, wronglogin_authdata.1)
        .await
        .is_err());
    // Asserts an error occured with a wrong url
    assert!(Mattermost::new("www.wrong-url.com", None)
        .login(good_authdata.0, good_authdata.1)
        .await
        .is_err());

    Ok(())
}

// Tests a post is correctly created on an existing Mattermost instance.
#[tokio::test]
async fn test_can_post() -> TestResult<()> {
    // Unit test preparation
    // Retrieves the user id and password in the environment variable.
    let password = std::env::var("pass").unwrap();
    let login = std::env::var("log").unwrap();

    let url = "https://sofairofficial.cloud.mattermost.com";
    let channel_id = "p1kfdiyg53gzjki1cpsfr4fzwe";

    let message = "Test to post a Release";
    let created_post = CreatePost::build(channel_id, message);

    let mut mattermost = Mattermost::new(url, None);
    mattermost.login(login, password).await?;

    // Asserts the message is correctly posted
    assert!(mattermost.post(created_post).await.is_ok());

    Ok(())
}
