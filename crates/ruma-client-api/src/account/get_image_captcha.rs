pub mod v3 {
    //! `/v3/` ([spec])
    //!
    //! [spec]: https://spec.matrix.org/latest/client-server-api/#post_matrixclientv3account3pidadd

    use ruma_common::{
        api::{request, response, Metadata},
        metadata,
    };

    use crate::uiaa::{AuthData, UiaaResponse};

    const METADATA: Metadata = metadata! {
        method: GET,
        rate_limited: true,
        authentication: None,
        history: {
            1.0 => "/_matrix/client/r0/register/token/captcha",
            1.1 => "/_matrix/client/v3/register/token/captcha",
        }
    };

    /// Request type for the `add_3pid` endpoint.
    #[request(error = crate::Error)]
    pub struct Request{
        #[ruma_api(query)]
        pub number:String
    }

    #[response(error = crate::Error)]
    #[derive(Default)]
    pub struct Response {
        pub image:String,
        pub token:String,
    }

    impl crate::account::get_image_captcha::v3::Request {
        /// Creates a new `Request` with the given client secret and session identifier.
        pub fn new(number:String) -> Self {
            Self {
                number:number,
            }
        }
    }

    impl crate::account::get_image_captcha::v3::Response {
        /// Creates an empty `Response`.
        pub fn new(image:String,token:String) -> Self {
            Self {token: token,image:image}
        }
    }
}
