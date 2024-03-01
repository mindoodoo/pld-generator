use std::{collections::BTreeMap, fmt};

use regress::{Flags, Regex};

use crate::github::card::ProjectCard;

// Section parsing regex's
const USER_WISH_REGEX: &str = r"(?<=^# *User wish$\s+)\S(?:.|\s)*?(?=\n+# *Description)";
const DESCRIPTION_REGEX: &str = r"(?<=^# *Description$\s+)\S(?:\s|.)*?(?=\s+# *DOD)";
const DOD_REGEX: &str = r"(?<=^# *DOD$\s+)\S(?:\s|.)*\S$";

// User wish parsing regex's
const USER_WISH_INTERIOR: &str =
    r"\*\*as the:\*\*\s+?(.+?$)(?:\s)+?\*\*i want to:\*\*((?:.|\s)+.+)";

// Flags to be used
const FLAGS: Flags = Flags {
    icase: true,
    multiline: true,
    dot_all: false,
    no_opt: false,
    unicode: false,
};

#[derive(Debug)]
pub enum CardSection {
    UserWish,
    Description,
    Dod,
}

#[derive(Debug)]
pub enum ParsingError {
    SectionMissing(CardSection),
    SectionContainsHeader(CardSection),
    SectionMissingInformation(CardSection),
    TooManyMatches(CardSection),
}

/// This is the first part of the card
///
/// > As the `user`, I want to `action`
#[derive(Debug)]
pub struct UserWish {
    pub user: String,
    pub action: String,
}

impl UserWish {
    pub fn from_markdown(user_wish: &str) -> Result<UserWish, ParsingError> {
        let user_regex = Regex::with_flags(USER_WISH_INTERIOR, FLAGS).unwrap();

        let matches = user_regex
            .find(user_wish)
            .ok_or(ParsingError::SectionMissingInformation(
                CardSection::UserWish,
            ))?;
        let user_group = matches
            .group(1)
            .ok_or(ParsingError::SectionMissingInformation(
                CardSection::UserWish,
            ))?;
        let action_group = matches
            .group(2)
            .ok_or(ParsingError::SectionMissingInformation(
                CardSection::UserWish,
            ))?;

        Ok(UserWish {
            user: user_wish[user_group].trim().to_string(),
            action: user_wish[action_group].trim().to_string(),
        })
    }
}

/// Main structure representing the parsed contents of a card
#[derive(Debug)]
pub struct PldCard {
    pub name: String,
    pub section: String,
    pub sub_section: String,
    pub wish: UserWish,
    pub description: String,
    pub dod: String,
    pub working_days: f32,
}

impl PldCard {
    pub fn new(card_resp: &ProjectCard) -> Result<PldCard, ParsingError> {
        let user_wish_regex = Regex::with_flags(USER_WISH_REGEX, FLAGS).unwrap();
        let description_regex = Regex::with_flags(DESCRIPTION_REGEX, FLAGS).unwrap();
        let dod_regex = Regex::with_flags(DOD_REGEX, FLAGS).unwrap();

        let wish = match user_wish_regex.find(&card_resp.content) {
            Some(m) => UserWish::from_markdown(&card_resp.content[m.range])?,
            None => return Err(ParsingError::SectionMissing(CardSection::UserWish)),
        };

        let description = match description_regex.find(&card_resp.content) {
            Some(m) => (&card_resp.content[m.range]).trim().to_string(),
            None => return Err(ParsingError::SectionMissing(CardSection::Description)),
        };

        let dod = match dod_regex.find(&card_resp.content) {
            Some(m) => (&card_resp.content[m.range]).trim().to_string(),
            None => return Err(ParsingError::SectionMissing(CardSection::Dod)),
        };

        Ok(PldCard {
            name: card_resp.name.clone(),
            section: card_resp.section.clone(),
            sub_section: card_resp.sub_section.clone(),
            wish,
            description,
            dod,
            working_days: card_resp.working_days,
        })
    }
}

impl fmt::Display for PldCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n\n", &self.name)?;
        write!(f, "**As a:** {}\n\n", self.wish.user)?;
        write!(f, "**I want to:** {}\n\n", self.wish.action)?;

        write!(f, "**Description**\n\n{}\n\n", &self.description)?;
        write!(f, "**Definition of Done**\n\n{}\n\n", self.dod)?;
        write!(f, "**Working days :** {}", self.working_days)?;

        Ok(())
    }
}

pub fn sort_by_section(
    mut cards: Vec<PldCard>,
) -> BTreeMap<String, BTreeMap<String, Vec<PldCard>>> {
    let mut output: BTreeMap<String, BTreeMap<String, Vec<PldCard>>> = BTreeMap::new();

    cards.sort_by_key(|card| card.name.clone());

    for card in cards {
        output
            .entry(card.section.clone())
            .or_default()
            .entry(card.sub_section.clone())
            .or_default()
            .push(card);
    }

    output
}
