#[macro_use] extern crate rocket;
extern crate rocket_dyn_templates;
use rocket_dyn_templates::Template; 
use rocket::fs::{FileServer, relative};
//use std::collections::HashMap;
use rocket::serde::ser::{Serialize, SerializeStruct, Serializer};

#[get("/")]
fn index() -> Template {
    struct Menu {
        url: String,
        name: String
    }
    impl Serialize for Menu {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, {
            let mut s = serializer.serialize_struct("Menu", 2)?;
            s.serialize_field("url", &self.url)?;
            s.serialize_field("name", &self.name)?;
            s.end()
        }
    }
    struct Page {
        blog_title: String,
        blog_header: String,
        menu_list: Vec<Menu>,
        content_id: String,
        content_title: String,
        content_text: String
    }
    impl Serialize for Page {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, {
            let mut s = serializer.serialize_struct("Page", 6)?;
            s.serialize_field("blog_title", &self.blog_title)?;
            s.serialize_field("blog_header", &self.blog_header)?;
            s.serialize_field("menu_list", &self.menu_list)?;
            s.serialize_field("content_id", &self.content_id)?;
            s.serialize_field("content_title", &self.content_title)?;
            s.serialize_field("content_text", &self.content_text)?;
            s.end()
        }
    }
    let context = Page {
        blog_title: "BLOG".to_string(),
        blog_header: "BLOG HEADER".to_string(),
        menu_list: vec![Menu{url: "/".to_string(), name: "Home".to_string()}, Menu{url: "about.html".to_string(), name: "About".to_string()}],
        content_id: "GREETING".to_string(),
        content_title: "This site is a WIP!".to_string(),
        content_text: "Not done yet! Please standby!".to_string()
    };
    Template::render("index", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", FileServer::from(relative!("static")))
        //.mount("/css", FileServer::from(relative!("static/css")))
        //.mount("/img", FileServer::from(relative("static/img")))
        .attach(Template::fairing())
}
