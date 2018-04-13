#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate failure;

use failure::Error;

use std::path::{Path, PathBuf};

use rocket::response::{self, Responder, content, NamedFile};
use rocket::request::Request;

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
            <h1>Today's pizza is {}</h1>
            <img src="{}" width="400"/>
            <p>{}</p>
        </body>
        </html>"#,
            self.name, self.name, self.image, self.description
        )
    }
}

impl<'a> Responder<'a> for Pizza {
 
    fn respond_to(self, req: &Request) -> response::Result<'a> {
        content::Html(self.html()).respond_to(req)
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
    kitchen("Hawaiian")
}

fn main() {
    rocket::ignite().mount("/", routes![index, images]).launch();
}
