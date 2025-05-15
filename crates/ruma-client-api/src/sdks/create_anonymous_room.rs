pub mod v0 {
    use ruma_common::{

        api::{request, response, Metadata},
        metadata,
    };

    const METADATA: Metadata = metadata! {
        method: POST,
        rate_limited: true,
        authentication: None,
        history: {
            1.0 => "/v0/sdk/api/index/room/none/create",
        }
    };
    #[request(error = crate::ApiError)]
    pub struct Request{
        pub name:String,
        pub topic:Option<String>,
    }

    impl Request {
        /// Creates a new `Request` with the given client secret and session identifier.
        pub fn new(name:String,topic:Option<String>) -> Self {
            Self {
                name:name,
                topic:topic,
            }
        }
    }

    #[response(error = crate::ApiError)]
    #[derive(Default)]
    pub struct Response {
        pub room_id:String,
        pub share_link:String,
    }

}