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
use sysinfo::{ProcessExt, SystemExt};
use systemstat::{ByteSize, Platform, System};

impl<'a> Responder<'a, 'a> for Error {
    fn respond_to(self, _: &'a Request<'_>) -> response::Result<'a> {
        let str = format!("{:?}", self);
        Response::build()
            .header(ContentType::Plain)
            .sized_body(str.len(), Cursor::new(str))
            .ok()
    }
}

fn get_proc_mem() -> Result<String, Error> {
    let pid = sysinfo::get_current_pid()?;
    let sys = sysinfo::System::new_all();
    let proc = sys
        .get_process(pid)
        .ok_or(Error::Custom("process not found"))?;

    Ok(format!("{}", ByteSize::kb(proc.memory()),))
}

fn get_sys_mem() -> Result<String, Error> {
    let sys = System::new();
    let mem = sys.memory()?;
    Ok(format!(
        "{}/{}",
        systemstat::saturating_sub_bytes(mem.total, mem.free),
        mem.total,
    ))
}

#[get("/")]
async fn hello(cache_http: State<'_, Box<dyn CacheHttp>>) -> Result<String, Error> {
    let tag = cache_http
        .cache()
        .ok_or(Error::Custom("expected cache"))?
        .current_user()
        .await
        .tag();

    let proc_mem = match get_proc_mem() {
        Ok(m) => m,
        Err(why) => format!("<error {:?}>", why),
    };

    let sys_mem = match get_sys_mem() {
        Ok(m) => m,
        Err(why) => format!("<error {:?}>", why),
    };

    Ok(format!(
        "Hello, I'm logged in as {}. RAM usage: proc {}, sys {}",
        tag, proc_mem, sys_mem
    ))
}

pub async fn run(_: &Config, cache_http: Box<dyn CacheHttp>) -> Result<(), rocket::Error> {
    rocket::build()
        .manage(cache_http)
        .mount("/", routes![hello])
        .launch()
        .await
}
