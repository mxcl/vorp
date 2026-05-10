pub enum LoginFailureReason {
    InvalidRedirectUrl { was_pasted: bool },
    FailedUserAuthentication,
    FailedMintCustomToken,
    InvalidStateParameter,
    MissingStateParameter,
}
