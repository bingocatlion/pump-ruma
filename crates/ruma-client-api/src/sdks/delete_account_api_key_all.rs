pub mod v3 {
    use ruma_common::{api::{request, response, Metadata}, metadata};
    use serde::{Deserialize, Serialize};
    use crate::sdks::create_account_api_key::v3::ApiKeyInfo;

    const METADATA: Metadata = metadata! {
        method: GET,
        rate_limited: true,
        authentication: AccessToken,
        history: {
            1.1 => "/_matrix/client/v3/apikey/deleteAll",
        }
    };

    #[request(error =  crate::Error)]
    pub struct Request{
    }

    impl Request {
        /// Creates a new `Request` with the given client secret and session identifier.
        pub fn new() -> Self {
            Self {

            }
        }
    }

    #[response(error = crate::Error)]
    #[derive(Default)]
    pub struct Response {
        pub keys:Vec<ApiKeyInfo>,
    }
}