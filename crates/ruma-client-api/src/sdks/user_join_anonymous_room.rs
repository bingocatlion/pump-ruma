pub mod v3 {
    use ruma_common::{

        api::{request, response, Metadata},
        metadata,
    };

    const METADATA: Metadata = metadata! {
        method: GET,
        rate_limited: false,
        authentication: None,
        history: {
            1.1 => "/_matrix/client/v3/user/anonymous/join/:room_id",
        }
    };

    #[request(error =  crate::Error)]
    pub struct Request{
        #[ruma_api(path)]
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