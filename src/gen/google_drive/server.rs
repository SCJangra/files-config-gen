use anyhow::Context;
use rocket::{Shutdown, State};
use std::{
    net::{IpAddr, Ipv4Addr},
    sync::mpsc::SyncSender,
};

pub fn start<T>(state: T, port: u16) -> anyhow::Result<()>
where
    T: Send + Sync + 'static,
{
    let c = rocket::Config {
        port,
        address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        log_level: rocket::config::LogLevel::Off,
        ..Default::default()
    };

    let f = rocket::figment::Figment::from(c);

    let r = rocket::custom(f)
        .manage(state)
        .mount("/", routes![on_res])
        .launch();

    let _ = rocket::execute(r).with_context(|| "Server unexpectedly stopped!")?;

    Ok(())
}

#[get("/?<code>&<error>&<error_description>")]
fn on_res(
    s: &State<SyncSender<String>>,
    shutdown: Shutdown,
    code: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
) -> String {
    if let Some(code) = code {
        s.send(code).unwrap();
        shutdown.notify();
        String::from("Done!")
    } else {
        let error = error.unwrap();

        if let Some(ed) = error_description {
            shutdown.notify();
            return format!("Error: {}\nDesc: {}", error, ed);
        }

        shutdown.notify();
        return format!("Error: {}", error);
    }
}
