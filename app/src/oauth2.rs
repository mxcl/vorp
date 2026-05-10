use url::Url;

#[derive(Clone, Debug, Default)]
pub struct StandardDeviceAuthorizationResponse {
    verification_uri: String,
    verification_uri_complete: Option<Secret>,
    user_code: Secret,
}

impl StandardDeviceAuthorizationResponse {
    pub fn verification_uri(&self) -> &str {
        &self.verification_uri
    }

    pub fn verification_uri_complete(&self) -> Option<&Secret> {
        self.verification_uri_complete.as_ref()
    }

    pub fn user_code(&self) -> &Secret {
        &self.user_code
    }
}

#[derive(Clone, Debug, Default)]
pub struct Secret(String);

impl Secret {
    pub fn secret(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct ClientId(String);

impl ClientId {
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug)]
pub struct TokenUrl(Url);

impl TokenUrl {
    pub fn from_url(value: Url) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug)]
pub struct DeviceAuthorizationUrl(Url);

impl DeviceAuthorizationUrl {
    pub fn from_url(value: Url) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug)]
pub struct EndpointNotSet;

#[derive(Clone, Debug)]
pub struct EndpointSet;

pub trait TokenResponse {
    fn access_token(&self) -> &Secret;
}

pub mod basic {
    use std::marker::PhantomData;

    use super::{ClientId, DeviceAuthorizationUrl, EndpointNotSet, EndpointSet, TokenUrl};

    #[derive(Clone, Debug)]
    pub struct BasicClient<
        HasAuthUrl,
        HasDeviceAuthUrl,
        HasIntrospectionUrl,
        HasRevocationUrl,
        HasTokenUrl,
    > {
        _marker: PhantomData<(
            HasAuthUrl,
            HasDeviceAuthUrl,
            HasIntrospectionUrl,
            HasRevocationUrl,
            HasTokenUrl,
        )>,
    }

    impl BasicClient<EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointNotSet> {
        pub fn new(_client_id: ClientId) -> Self {
            Self {
                _marker: PhantomData,
            }
        }
    }

    impl<HasAuthUrl, HasDeviceAuthUrl, HasIntrospectionUrl, HasRevocationUrl, HasTokenUrl>
        BasicClient<
            HasAuthUrl,
            HasDeviceAuthUrl,
            HasIntrospectionUrl,
            HasRevocationUrl,
            HasTokenUrl,
        >
    {
        pub fn set_token_uri(
            self,
            _url: TokenUrl,
        ) -> BasicClient<
            HasAuthUrl,
            HasDeviceAuthUrl,
            HasIntrospectionUrl,
            HasRevocationUrl,
            EndpointSet,
        > {
            BasicClient {
                _marker: PhantomData,
            }
        }

        pub fn set_device_authorization_url(
            self,
            _url: DeviceAuthorizationUrl,
        ) -> BasicClient<HasAuthUrl, EndpointSet, HasIntrospectionUrl, HasRevocationUrl, HasTokenUrl>
        {
            BasicClient {
                _marker: PhantomData,
            }
        }
    }
}
