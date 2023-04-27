//! Sleppa commit analyzer package
//!
//! This crate reads the commit's messages and matches a release action type.
//!
//! Thanks to the crate [sleppa_configuration], the [Configuration] contains the [ReleaseRule] to apply
//! for each [ReleaseAction]. These can be used to parsed the commit's messages.
//!
//! In order to match a new [ReleaseAction], the commit's message from the last tag must be retrieved.
//! They will be stored in a structure [MessagesToAnalyze].
//!
//! The trait [CommitHandler] is used to bring the commit's messages to the [CommitAnalyzer] with the
//! [MessagesToAnalyze] structure.
//!
//! As only one release action type must be defined for a new release, only the higher one is kept :
//! - Major > Minor > Patch

mod errors;
//mod github;

use errors::*;
use sleppa_configuration::*;

#[derive(Debug, Default)]
pub struct CommitAnalyzer {}

/// Handles all the messages that must be analyzed
///
/// It is used to store all the commit's message retrieved since the last tag of a repository
#[derive(Debug, Default)]
pub struct MessagesToAnalyze {
    messages: Vec<String>,
}

impl CommitAnalyzer {
    /// Parses a message and matches a ReleaseAction
    ///
    /// This function reads a provided message and verify if the message matches a ReleaseAction
    /// thanks to the trait [ReleaseRuleHandler].
    /// If no match is found, a [CommitAnalyzerError] is returned.
    pub fn execute(&self, message: &str, release_rule: &ReleaseRules) -> CommitAnalyzerResult<ReleaseAction> {
        if release_rule[&ReleaseAction::Major].handle(message).is_ok() {
            Ok(ReleaseAction::Major)
        } else if release_rule[&ReleaseAction::Minor].handle(message).is_ok() {
            Ok(ReleaseAction::Minor)
        } else if release_rule[&ReleaseAction::Patch].handle(message).is_ok() {
            Ok(ReleaseAction::Patch)
        } else {
            Err(CommitAnalyzerError::ErrorNoMatching())
        }
    }

    /// Verifies multiple commit's message to retrieve the higher release action type to apply
    ///
    /// This function receives [MessageToAnalyze] struct and analyzes them to retrieve the release action
    /// type to apply since the last tag.
    /// As it is impossible to have two release action types at the same time, only the higher one is kept.
    pub fn action_to_release(&self, mes: MessagesToAnalyze, release_rule: &ReleaseRules) -> Option<ReleaseAction> {
        let mut major_count = 0;
        let mut minor_count = 0;
        let mut patch_count = 0;
        let mut _no_type = 0;
        for message in mes.messages {
            match self.execute(&message, release_rule) {
                Ok(ReleaseAction::Major) => major_count += 1,
                Ok(ReleaseAction::Minor) => minor_count += 1,
                Ok(ReleaseAction::Patch) => patch_count += 1,
                _ => _no_type += 1,
            }
        }
        if major_count > 0 {
            Some(ReleaseAction::Major)
        } else if minor_count > 0 {
            Some(ReleaseAction::Minor)
        } else if patch_count > 0 {
            Some(ReleaseAction::Patch)
        } else {
            None
        }
    }
}

pub trait CommitHandler {
    fn get_messages_from_last_tag(&self) -> MessagesToAnalyze;
}

#[cfg(test)]
mod tests;
