use primitives::grant::Grant;
use primitives::issuer::{IssuedToken, Issuer};

use super::super::actix::{Handler, Message};
use super::super::AsActor;

/// Command issuing a bearer token.
pub struct Issue {
    /// The grant defining client, scope and timeframe of the token.
    pub grant: Grant,
}

impl Message for Issue {
    type Result = Result<IssuedToken, ()>;
}

/// Try to find the grant of a specific token.
pub struct RecoverToken {
    /// Previously returned as `token` in an `IssuedToken`.
    pub token: String,
}

impl Message for RecoverToken {
    /// An `Ok` represents a successful search (which may find nothing).
    type Result = Result<Option<Grant>, ()>;
}

/// Try to find the refresh grant of a specific token.
pub struct RecoverRefresh {
    /// Previously returned as `refresh` in an `IssuedToken`.
    pub token: String,
}

impl Message for RecoverRefresh {
    /// An `Ok` represents a successful search (which may find nothing).
    type Result = Result<Option<Grant>, ()>;
}

impl<I: Issuer + 'static> Handler<Issue> for AsActor<I> {
    type Result = Result<IssuedToken, ()>;

    fn handle(&mut self, msg: Issue, _: &mut Self::Context) -> Self::Result {
        self.0.issue(msg.grant)
    }
}

impl<I: Issuer + 'static> Handler<RecoverToken> for AsActor<I> {
    type Result = Result<Option<Grant>, ()>;

    fn handle(&mut self, msg: RecoverToken, _: &mut Self::Context) -> Self::Result {
        self.0.recover_token(&msg.token)
    }
}

impl<I: Issuer + 'static> Handler<RecoverRefresh> for AsActor<I> {
    type Result = Result<Option<Grant>, ()>;

    fn handle(&mut self, msg: RecoverRefresh, _: &mut Self::Context) -> Self::Result {
        self.0.recover_refresh(&msg.token)
    }
}
