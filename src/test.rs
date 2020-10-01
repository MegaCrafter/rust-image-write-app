use super::rocket;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn index_page() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/").dispatch();

    // Is index page working?
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn github_page() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/github").dispatch();

    let location = response.headers().get_one("Location");

    // Is github page redirecting correctly? 
    // (Probably not needed but just to make sure...)
    assert_eq!(response.status(), Status::SeeOther);
    assert!(location.is_some());
    assert_eq!(location.unwrap(), "https://github.com/MegaCrafter/rust-image-write-app");
}

// TODO: Add a test to test response images