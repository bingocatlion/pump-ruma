pub mod v3 {
    use ruma_common::{api::{request, response, Metadata}, metadata};
    use serde::{Deserialize, Serialize};

    const METADATA: Metadata = metadata! {
        method: GET,
        rate_limited: true,
        authentication: AccessToken,
        history: {
            1.1 => "/_matrix/client/v3/apikey/create",
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

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    #[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
    pub struct ApiKeyInfo {
        /// A list of user sessions on this device.
        pub apikey: String,
        pub user_id: i64,
        pub expires_at: i32,
        pub is_disabled:Option<bool>,
        pub limit_temp_rooms:i32,
        pub limit_chats:i32,
        pub limit_users:i32,
        pub created_at:i32,
        pub name:String,
        pub auto_join_rooms:Vec<String>,
        pub auto_join_user_id_list:Vec<String>,
    }


    #[response(error = crate::Error)]
    #[derive(Default)]
    pub struct Response {
        pub keys:Vec<ApiKeyInfo>,
    }
}