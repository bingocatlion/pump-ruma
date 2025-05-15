pub mod v0 {
    use ruma_common::{

        api::{request, response, Metadata},
        metadata,
    };

    const METADATA: Metadata = metadata! {
        method: POST,
        rate_limited: false,
        authentication: None,
        history: {
            1.0 => "/v0/sdk/api/user/room/none/join",
        }
    };
    #[request(error =  crate::Error)]
    pub struct Request{
        pub room_id:String,
    }

    impl Request {
        /// Creates a new `Request` with the given client secret and session identifier.
        pub fn new(room_id:String) -> Self {
            Self {
                room_id:room_id,
            }
        }
    }

    #[response(error = crate::Error)]
    #[derive(Default)]
    pub struct Response {
        pub room_id:String,
    }

}