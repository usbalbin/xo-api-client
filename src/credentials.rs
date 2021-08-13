use std::{collections::BTreeMap, str::FromStr};

use crate::types::Impossible;
use jsonrpsee_types::JsonValue;

/// Login token used to authenticate with Xen Orchestra's API
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct Token(pub String);

impl ToString for Token {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl FromStr for Token {
    type Err = Impossible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Token(s.to_string()))
    }
}

impl From<Token> for Credentials {
    fn from(val: Token) -> Self {
        Credentials::Token(val)
    }
}

/// Email and password used to authenticate with Xen Orchestra's API.
///
/// Note that there is also the type [`Token`]
#[derive(Debug, Clone)]
pub struct EmailAndPassword {
    pub email: String,
    pub password: String,
}

impl From<EmailAndPassword> for Credentials {
    fn from(val: EmailAndPassword) -> Self {
        Credentials::Password(val)
    }
}

/// Some type of credentials used to authenticate with Xen Orchestra's API.
///
/// A value of this type may ether contain a [`Token`] or an [`EmailAndPassword`]
pub enum Credentials {
    Password(EmailAndPassword),
    Token(Token),
}

impl From<Credentials> for BTreeMap<&str, JsonValue> {
    fn from(credentials: Credentials) -> Self {
        use std::array::IntoIter;

        match credentials {
            Credentials::Password(EmailAndPassword { email, password }) => {
                IntoIter::new([("email", email.into()), ("password", password.into())]).collect()
            }
            Credentials::Token(Token(token)) => IntoIter::new([("token", token.into())]).collect(),
        }
    }
}
