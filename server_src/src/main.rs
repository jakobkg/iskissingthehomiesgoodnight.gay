use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use lazy_static::lazy_static;
use rand;
use regex::Regex;
use serde::Deserialize;
use std::fs;

// SSL dependencies should only be pulled in if we're building with HTTPS support
#[cfg(feature = "production")]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

// Struct holding the provided personalia of a contributor
#[derive(Debug, Deserialize)]
struct Personalia {
    displayname: Option<String>,
    github: String,
    twitter: Option<String>,
}

// Struct holding the personalia and the list of styles provided by a contributor
#[derive(Debug, Deserialize)]
struct Contributor {
    personalia: Personalia,
    styles: Vec<String>,
}

// The outermost Contributor struct, holding the list of Contributors
#[derive(Debug, Deserialize)]
struct ContributorStruct {
    contributors: Vec<Contributor>,
}

// Read the HTML template and Contributor JSON on server launch
// These are important, and the server will not launch without them
lazy_static! {
    static ref TEMPLATE: String =
        fs::read_to_string("index.html").expect("Critical: Unable to read/load HTML template!");
    static ref CONTRIBUTORS: Vec<Contributor> = serde_json::from_str(
        &fs::read_to_string("contributors.json")
            .expect("Critical: Unable to read/load contributor JSON")
    )
    .unwrap_or(ContributorStruct {
        contributors: Vec::new(),
    })
    .contributors;
}

// Sets the URL root for links to static assets such as icons based on whether we're compiling for production or not
static CNAME: &str = if cfg!(feature = "production") {
    "https://iskissingthehomiesgoodnight.gay"
} else {
    "http://localhost:8080"
};

/// Constructs a <div> containing a logo and a username for a given service
/// Currently only works with a very specific URL format, but Github and Twitter both fit this
fn make_div<S: Into<String> + std::fmt::Display>(service: S, name: S) -> String {
    format!("<div><a href=\"https://{service}.com/{name}\"><img src=\"{CNAME}/assets/{service}.png\"> {name}</a></div>")
}

/// URL handler for the root of the webpage
///
/// This function creates a copy of the HTML template, loads a randomly chosen style and
/// injects it into the HTML document along with optional attribution links before
/// serving the page back to the client that requested it.
#[get("/")]
async fn front() -> impl Responder {
    // RegEx parsers for the <script> and <style> injection points
    let script_re = Regex::new("/\\* script insert \\*/").unwrap();
    let style_re = Regex::new("/\\* style insert \\*/").unwrap();

    // RegEx parsers for the Github, Twitter and display name injection points
    let name_re = Regex::new("<!-- name insert -->").unwrap();
    let gh_re = Regex::new("<!-- github insert -->").unwrap();
    let twitter_re = Regex::new("<!-- twitter insert -->").unwrap();

    // Load a mutable copy of the HTML template
    let mut template = TEMPLATE.clone();

    // Select a random contributor from the contributors list
    let contributor = &CONTRIBUTORS[rand::random::<usize>() % CONTRIBUTORS.len()];

    // Select a random style from the chosen contributor
    let stylename = &contributor.styles[rand::random::<usize>() % contributor.styles.len()];

    // Store the Github username of the chosen contributor in a variable for ease of use,
    // then inject a link to this github user into the attribution section of the template
    let gh = &contributor.personalia.github;
    template = gh_re.replace(&template, make_div("github", gh)).into();

    // If a display name has been provided for the contributor, inject it into the template
    match &contributor.personalia.displayname {
        Some(displayname) => {
            template = name_re
                .replace(&template, format!("Style by {displayname}"))
                .into();
        }
        None => {}
    }

    // If a Twitter handle has been provided for the contributor, inject a link into the template
    match &contributor.personalia.twitter {
        Some(twitter) => {
            template = twitter_re
                .replace(&template, make_div("twitter", twitter))
                .into()
        }
        None => {}
    }

    // Load the provided stylesheet into the template, if any
    let style = fs::read_to_string(format!("dist/styles/{gh}/{stylename}.css")).unwrap_or_default();

    // Load the provided script into the template, if any
    let script =
        fs::read_to_string(format!("dist/scripts/{gh}/{stylename}.js")).unwrap_or_default();

    // If we're in a dev environment, print a lil' thing to the terminal
    if !cfg!(feature = "production") {
        println!("Responding with style {stylename} by {gh}");
    }

    // Inject the <style> and <script> content into the template, and return it to the client
    template = style_re
        .replace(&script_re.replace(&template, script), style)
        .into();

    HttpResponse::Ok().body(template)
}

/// Get request handler that responds with a specific style rather than randomly choosing one
/// Meant to make developing a specific style easier as you're no longer at the whim of the RNG
#[cfg(not(feature = "production"))]
async fn get_style(params: web::Path<(String, String)>) -> impl Responder {
    let script_re = Regex::new("/\\* script insert \\*/").unwrap();
    let style_re = Regex::new("/\\* style insert \\*/").unwrap();

    // Clone the template
    let mut template = TEMPLATE.clone();

    // Load the provided stylesheet into the template, if any
    let style = fs::read_to_string(format!("dist/styles/{}/{}.css", params.0, params.1)).unwrap_or_default();

    // Load the provided script into the template, if any
    let script =
        fs::read_to_string(format!("dist/scripts/{}/{}.js", params.0, params.1)).unwrap_or_default();
    
    template = style_re
        .replace(&script_re.replace(&template, script), style)
        .into();

    HttpResponse::Ok().body(template)
}

// Main method used if the server is built in production mode, with SSL
#[cfg(feature = "production")]
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
    .bind_openssl("iskissingthehomiesgoodnight.gay:443", builder)?
    .run()
    .await
}

// Main method used if the server is not built in production mode, hosting on localhost without HTTPS
#[cfg(not(feature = "production"))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting, try visiting http://localhost:8080 to see the results");
    HttpServer::new(|| {
        App::new()
            .service(front)
            .service(Files::new("/assets", "./assets"))
            .route(
                "/{name}/{contribution}",
                web::get().to(get_style)
            )
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
