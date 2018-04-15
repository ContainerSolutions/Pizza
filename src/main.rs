#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate failure;
extern crate ctrlc;

use failure::Error;

use std::path::{Path, PathBuf};

use rocket::config::{Config, Environment};
use rocket::fairing;
use rocket::http::hyper::header::Pragma;
use rocket::request::Request;
use rocket::response::{self, content, NamedFile, Responder};
use std::env;

pub struct Pizza {
    name: String,
    description: String,
    image: String,
}

impl Pizza {
    fn html(&self) -> String {
        format!(
            r#"
        <!DOCTYPE html>
        <html lang=en>
        <head>
            <meta charset=utf-8>
            <title>{}</title>
        </head>
        <body>
            <h1>Today's Pizza is {}</h1>
            <img src="{}" width="400"/>
            <p>{}</p>
        </body>
        </html>
"#,
            self.name, self.name, self.image, self.description
        )
    }
}

impl<'a> Responder<'a> for Pizza {
    fn respond_to(self, req: &Request) -> response::Result<'a> {
        let mut res = content::Html(self.html()).respond_to(req)?;
        res.set_header(Pragma::NoCache);
        Ok(res)
    }
}

pub fn kitchen(pizza: &str) -> Result<Pizza, Error> {
    let pizza = pizza.to_lowercase();
    match pizza.as_ref() {
        "margherita" => Ok(Pizza {
            name: "Margherita".to_owned(),
            description: "Basically cheese and tomato".to_owned(),
            image: "./img/marg.jpg".to_owned(),
        }),
        "hawaiian" => Ok(Pizza {
            name: "Hawaiian".to_owned(),
            description: "Ham and pineapple, the most controvesial of pizzas".to_owned(),
            image: "./img/hawaiian.jpg".to_owned(),
        }),
        "four seasons" => Ok(Pizza {
            name: "Four Seasons".to_owned(),
            description: "Olives and stuff I think".to_owned(),
            image: "./img/four.jpg".to_owned(),
        }),
        "quattro formaggi" => Ok(Pizza {
            name: "Quattro Formaggi".to_owned(),
            description: "Cheese, cheese, cheese and more cheese".to_owned(),
            image: "./img/quattro.jpg".to_owned(),
        }),
        _ => Err(format_err!("We don't know how to make {} pizza", pizza)),
    }
}

#[get("/img/<file..>")]
fn images(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("img/").join(file)).ok()
}

#[get("/")]
pub fn index() -> Result<Pizza, Error> {
    match env::var("PIZZA") {
        Ok(flavour) => {
            kitchen(&flavour)
        }
        _ => kitchen("Hawaiian"),
    }
}

fn attach_sigterm(r: rocket::Rocket) -> Result<rocket::Rocket, rocket::Rocket> {
    match ctrlc::set_handler(|| {
        println!("SIGTERM caught, shutting down...");
        std::process::exit(0);
    }) {
        Ok(_) => Ok(r),
        Err(_) => Err(r)
    }
}

fn main() {
    let config = Config::build(Environment::Production)
        .address("0.0.0.0")
        .port(8000)
        .finalize()
        .unwrap();

    let app = rocket::custom(config, false).attach(fairing::AdHoc::on_attach(attach_sigterm));
    app.mount("/", routes![index, images]).launch();
}
