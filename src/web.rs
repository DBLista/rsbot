use crate::config::Config;
use crate::rocket::{get, State};
use crate::utils::Error;
use rocket::http::ContentType;
use rocket::{
    response::{self, Responder},
    Request, Response,
};
use serenity::http::CacheHttp;
use std::io::Cursor;

impl<'a> Responder<'a, 'a> for Error {
    fn respond_to(self, _: &'a Request<'_>) -> response::Result<'a> {
        let str = format!("{:?}", self);
        Response::build()
            .header(ContentType::Plain)
            .sized_body(str.len(), Cursor::new(str))
            .ok()
    }
}

#[get("/")]
async fn hello(cache_http: State<'_, Box<dyn CacheHttp>>) -> Result<String, Error> {
    let user = cache_http
        .cache()
        .ok_or(Error::Custom("expected cache"))?
        .current_user()
        .await
        .tag();
    Ok(format!("Hello, I'm {}", user))
}

pub async fn run(_: &Config, cache_http: Box<dyn CacheHttp>) -> Result<(), rocket::Error> {
    rocket::build()
        .manage(cache_http)
        .mount("/", routes![hello])
        .launch()
        .await
}
