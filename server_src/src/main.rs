use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use actix_files::Files;
use regex::Regex;
use std::fs;
use serde::Deserialize;
use rand;
use lazy_static::lazy_static;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[derive(Debug, Deserialize)]
struct Personalia {
    displayname: Option<String>,
    github: String,
    twitter: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Contributor {
    personalia: Personalia,
    styles: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ContributorStruct {
    contributors: Vec<Contributor>,
}

lazy_static! {
    static ref TEMPLATE: String = fs::read_to_string("index.html")
        .expect("Burde kunnet lese denne");
    static ref CONTRIBUTORS: Vec<Contributor> = match serde_json::from_str(&fs::read_to_string("contributors.json")
        .expect("Burde kunnet lese denne")) {
        Ok(s) => {s},
        Err(_) => {ContributorStruct {
            contributors: Vec::new(),
        }},
    }.contributors;
}

static CNAME: &str = if cfg!(debug_assertions) {"https://localhost:8080"} else {"https://iskissingthehomiesgoodnight.gay"};


fn make_div<S: Into<String> + std::fmt::Display>(service: S, name: S) -> String {
    format!("<div><a href=\"https://{service}.com/{name}\"><img src=\"{CNAME}/assets/{service}.png\">: {name}</a></div>")
}

#[get("/")]
async fn front() -> impl Responder {
    let script_re = Regex::new("/\\* script insert \\*/").unwrap();
    let style_re = Regex::new("/\\* style insert \\*/").unwrap();
    
    let name_re = Regex::new("<!-- name insert -->").unwrap();
    let gh_re = Regex::new("<!-- github insert -->").unwrap();
    let twitter_re = Regex::new("<!-- twitter insert -->").unwrap();

    let mut template = TEMPLATE.clone();

    let contributor = &CONTRIBUTORS[rand::random::<usize>() % CONTRIBUTORS.len()];
    let stylename = &contributor.styles[rand::random::<usize>() % contributor.styles.len()];
    let gh = &contributor.personalia.github;

    template = gh_re.replace(&template, make_div("github", gh)).to_string();

    match &contributor.personalia.displayname {
        Some(displayname) => {template = name_re.replace(&template, format!("Style by {displayname}")).to_string()},
        None => {},
    }

    match &contributor.personalia.twitter {
        Some(twitter) => {template = twitter_re.replace(&template, make_div("twitter", twitter)).to_string()},
        None => {},
    }

    let style = match fs::read_to_string(format!("dist/styles/{gh}/{stylename}.css")) {
        Ok(content) => {content},
        Err(_) => {"".to_owned()},
    };

    let script = match fs::read_to_string(format!("dist/scripts/{gh}/{stylename}.js")) {
        Ok(content) => {content},
        Err(_) => {"".to_owned()},
    };

    println!("Responding with style {stylename} by {gh}");

    let inserted = style_re.replace(&script_re.replace(&template, script), style).to_string();
    HttpResponse::Ok().body(inserted)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    println!("Server starting...");
    HttpServer::new(|| {
        App::new()
            .service(front)
            .service(Files::new("/assets", "./assets"))
    })
    .bind_openssl(
        if cfg!(debug_assertions) {
            "localhost:8080"
        } else {
            "iskissingthehomiesgoodnight.gay:443"
        }, builder)?
    .run()
    .await
}
