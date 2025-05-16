pub mod v3 {
    use ruma_common::{
        api::{request, response, Metadata},
        metadata,
    };

    const METADATA: Metadata = metadata! {
        method: POST,
        rate_limited: false,
        authentication: None,
        history: {
            1.1 => "/_matrix/client/v3/anonymous/join",
        }
    };
    #[request(error =  crate::Error)]
    pub struct Request{
        pub room_id:String,
        pub password:string,
        pub user_id:Option<String>,
        pub first_name:Option<String>,
        pub last_name:Option<String>,
    }

    impl Request {
        /// Creates a new `Request` with the given client secret and session identifier.
        pub fn new(
            room_id: String,
            password: String,
            user_id: Option<String>,
            first_name: Option<String>,
            last_name: Option<String>
        ) -> Self {
            Self {
                room_id,
                password,
                user_id,
                first_name,
                last_name,
            }
        }
    }

    #[response(error = crate::Error)]
    #[derive(Default)]
    pub struct Response {
        pub room_id: String,
        pub user_id: String,       // 用户名
        pub device_id: String,     // 设备ID
        pub access_token: String,  // 登录token
        pub password: String,      // 登录密码
    }

}