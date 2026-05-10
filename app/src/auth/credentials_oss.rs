use super::user::FirebaseAuthTokens;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthOwnerType {
    User,
    Team,
}

#[derive(Clone, Debug)]
pub enum Credentials {
    Firebase(FirebaseAuthTokens),
    ApiKey {
        key: String,
        owner_type: Option<AuthOwnerType>,
    },
    SessionCookie,
    #[cfg(any(test, feature = "integration_tests", feature = "skip_login"))]
    Test,
}

impl Credentials {
    pub fn as_firebase(&self) -> Option<&FirebaseAuthTokens> {
        match self {
            Self::Firebase(tokens) => Some(tokens),
            Self::ApiKey { .. } | Self::SessionCookie => None,
            #[cfg(any(test, feature = "integration_tests", feature = "skip_login"))]
            Self::Test => None,
        }
    }

    pub fn as_api_key(&self) -> Option<&str> {
        match self {
            Self::ApiKey { key, .. } => Some(key),
            Self::Firebase(_) | Self::SessionCookie => None,
            #[cfg(any(test, feature = "integration_tests", feature = "skip_login"))]
            Self::Test => None,
        }
    }

    pub fn api_key_owner_type(&self) -> Option<AuthOwnerType> {
        match self {
            Self::ApiKey { owner_type, .. } => *owner_type,
            Self::Firebase(_) | Self::SessionCookie => None,
            #[cfg(any(test, feature = "integration_tests", feature = "skip_login"))]
            Self::Test => None,
        }
    }

    pub fn refresh_token(&self) -> Option<&str> {
        match self {
            Self::Firebase(tokens) => Some(&tokens.refresh_token),
            Self::ApiKey { .. } | Self::SessionCookie => None,
            #[cfg(any(test, feature = "integration_tests", feature = "skip_login"))]
            Self::Test => None,
        }
    }

    pub fn bearer_token(&self) -> AuthToken {
        match self {
            Self::Firebase(tokens) => AuthToken::Firebase(tokens.id_token.clone()),
            Self::ApiKey { key, .. } => AuthToken::ApiKey(key.clone()),
            Self::SessionCookie => AuthToken::NoAuth,
            #[cfg(any(test, feature = "integration_tests", feature = "skip_login"))]
            Self::Test => AuthToken::NoAuth,
        }
    }

    pub fn login_token(&self) -> Option<LoginToken> {
        None
    }
}

#[derive(Debug, Clone)]
pub enum AuthToken {
    Firebase(String),
    ApiKey(String),
    NoAuth,
}

impl AuthToken {
    pub fn as_bearer_token(&self) -> Option<&str> {
        match self {
            Self::Firebase(token) | Self::ApiKey(token) => Some(token),
            Self::NoAuth => None,
        }
    }

    pub fn bearer_token(&self) -> Option<String> {
        match self {
            Self::Firebase(token) | Self::ApiKey(token) => Some(token.clone()),
            Self::NoAuth => None,
        }
    }
}

#[derive(Debug)]
pub enum LoginToken {
    Firebase(FirebaseToken),
    ApiKey(String),
    SessionCookie,
}

#[derive(Debug)]
pub enum FirebaseToken {
    Refresh(RefreshToken),
    Custom(String),
}

impl FirebaseToken {
    pub fn access_token_url(&self, _api_key: &str) -> String {
        String::new()
    }

    pub fn access_token_request_body(&self) -> Vec<(&str, &str)> {
        Vec::new()
    }

    pub fn proxy_url(&self, _server_root: &str, _api_key: &str) -> String {
        String::new()
    }
}

#[derive(Debug, Clone)]
pub struct RefreshToken(String);

impl RefreshToken {
    pub fn new(token: impl Into<String>) -> Self {
        Self(token.into())
    }

    pub fn get(&self) -> &str {
        self.0.as_str()
    }
}
