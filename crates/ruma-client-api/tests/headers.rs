#![cfg(feature = "client")]

use http::HeaderMap;
use ruma_client_api::discovery::discover_homeserver;
use ruma_client_api::account::{request_3pid_management_token_via_email,get_image_captcha};
use ruma_common::ClientSecret;
use ruma_common::api::{MatrixVersion, OutgoingRequest as _, SendAccessToken};

#[test]
fn get_request_headers() {
    let req: http::Request<Vec<u8>> = discover_homeserver::Request::new()
        .try_into_http_request(
            "https://homeserver.tld",
            SendAccessToken::None,
            &[MatrixVersion::V1_1],
        )
        .unwrap();

    assert_eq!(*req.headers(), HeaderMap::default());
}

#[test]
fn get_email_headers() {
    let client_secret = ClientSecret::new();
    let req = request_3pid_management_token_via_email::v3::Request::new_v2(client_secret,
                                                                 "a@gmail.com".to_string(),
                                                                     1.try_into().unwrap(),
                                                                           Some("aa".to_string()),Some("bb".to_string()));
    let req: http::Request<Vec<u8>> = req
        .try_into_http_request(
            "https://homeserver.tld",
            SendAccessToken::None,
            &[MatrixVersion::V1_1],
        )
        .unwrap();
    println!("============>>>>aaa-------->>>");
    println!("{:?}", req.headers());
    assert_eq!(*req.headers(), HeaderMap::default());
}

#[test]
fn get_captcha_headers() {

    let req = get_image_captcha::v3::Request::new("abbbb".to_string());
    let req: http::Request<Vec<u8>> = req
        .try_into_http_request(
            "https://homeserver.tld",
            SendAccessToken::None,
            &[MatrixVersion::V1_1],
        )
        .unwrap();

    println!("============>>>>aaa-------->>>");
    println!("{:?}", req.headers());
    assert_eq!(*req.headers(), HeaderMap::default());
}
