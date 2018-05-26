extern crate actix_web;
extern crate fragmentary;
extern crate futures;
extern crate serde_json;

use futures::prelude::*;

use actix_web::test::TestServer;

fn get_server() -> TestServer {
    TestServer::with_factory(fragmentary::app_factory)
}

#[test]
fn texts_index() {
    let mut srv = get_server();
    let req = srv.get().uri(srv.url("/texts")).finish().unwrap();
    let res = srv.execute(req.send()).unwrap();
    let (bytes, res) = srv.execute(res.into_future()).unwrap();
    assert!(res.status().is_success());
    let _text: Vec<fragmentary::model::text::Text> = serde_json::from_slice(
        &bytes.expect("response has a body")
    ).expect("could not deserialize");
}

#[test]
fn texts_command() {
    let mut srv = get_server();
    let payload = fragmentary::model::text::CreateText {
        text: Some(fragmentary::model::text::Text {
            id: "".to_string(),
            title: "tarahumaras".to_string(),
            authors: "antonin artaud".to_string(),
        }),
    };
    let req = srv
        .post()
        .uri(srv.url("/texts"))
        .json(payload.clone())
        .unwrap();
    let res = srv.execute(req.send()).unwrap();
    let (bytes, res) = srv.execute(res.into_future()).unwrap();
    assert!(res.status().is_success());
    let text: fragmentary::model::text::Text = serde_json::from_slice(
        &bytes.expect("response has a body")
    ).expect("could not deserialize");

    assert_eq!(text, payload.text.unwrap());
}
