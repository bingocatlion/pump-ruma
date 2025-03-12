//! `POST /registration/(?P<medium>[^/]*)/submit_token$`
//!
//! Get vaild regisgter


pub mod v3 {
use ruma_common::{
    api::{request, response, Metadata},
    metadata,
};


use crate::uiaa::{AuthData, UiaaResponse};

const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: true,
    authentication: None,
    history: {
        1.0 => "/_matrix/client/unstable/registration/email/submit_token",
        1.1 => "/_matrix/client/unstable/registration/email/submit_token",
    }
};


    /// Request type for the `add_3pid` endpoint.
    #[request(error = crate::Error)]
    pub struct Request{
        pub client_secret:String,
        pub sid:String,
        pub token:String
    }

    #[response(error = crate::Error)]
    #[derive(Default)]
    pub struct Response {
        pub token:Option<String>,
    }
    impl Request {
        /// Creates a new `Request` with the given client secret and session identifier.
        pub fn new(client_secret:String,sid:String,token:String) -> Self {
            Self {
                client_secret: client_secret,
                sid: sid,
                token: token,
            }
        }
    }

    impl Response {
        /// Creates an empty `Response`.
        pub fn new(token:Option<String>) -> Self {
            Self {token: token}
        }
    }
}