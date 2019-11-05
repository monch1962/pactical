//! This tool processes Pact contracts into ... well, just about anything!
//! The intended use case is to take a Pact contract and convert it into executable test cases 
//! (i.e. to test the provider) and/or executable stubs (i.e. provider mocks, to test a consumer against the mock)
//! 
//! Everything is done using Handlebars templates. Handlebars is a generic templating language, often used for generating
//! HTML content but useful for a broad range of tasks
//! 
//! Absolutely no limitations on what can be done with this approach. Some other "left field" ideas would include:
//! - generate PDF documentation of Pact coverage for human viewing &/or approval
//! 
extern crate serde_json;
#[macro_use]
extern crate handlebars;
extern crate serde;

// #[macro_use]
extern crate serde_derive;

use handlebars::Handlebars;
// use serde_json::json;
// use std::fs;
// use std::io::{self, Read};
use std::env;

use serde::{Deserialize, Serialize};
// use serde_json::Value as JsonValue;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process;

// extern crate serialize;
// use serialize::json;

extern crate chrono;
use chrono::Local;

extern crate inflector;
use inflector::Inflector;

#[derive(Serialize, Deserialize, Debug)]
struct Consumer {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Provider {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PactSpecification {
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    pact_specification: PactSpecification,
}

#[derive(Serialize, Deserialize, Debug)]
struct Header {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct HeaderString {
    header: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Message {
    description: String,
    provider_state: String,
    contents: String,
    meta_data: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Messages {
    message: Vec<Option<Message>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Request {
    method: String,
    path: String,
    headers: Option<serde_json::Value>,
    query: Option<serde_json::Value>,
    body: Option<serde_json::Value>,
    matching_rules: Option<serde_json::Value>,
    generators: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Response {
    status: Option<u16>,
    headers: Option<serde_json::Value>,
    body: Option<serde_json::Value>,
    generators: Option<serde_json::Value>,
    matching_rules: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Interaction {
    description: String,
    provider_state: Option<serde_json::Value>,
    request: Option<Request>,
    response: Option<Response>,
    messages: Option<Messages>
}

#[derive(Serialize, Deserialize, Debug)]
struct Pact {
    consumer: Consumer,
    provider: Provider,
    interactions: Vec<Interaction>,
    metadata: Metadata,
}

/// Read template file from the specified environment variable
/// and return the template as a string
fn read_template_file(template_env_var: String) -> String {
    let template = template_env_var;
    println!("Template env var: {}", template);

    let template_filename = format!("./templates/{}.hbs", template);
    let res = File::open(template_filename);
    if res.is_err() {
        eprintln!("{:#?}", res);
        eprintln!("Template file {} not found", template);
        process::exit(101);
    }
    let mut f = res.unwrap();

    let mut t = String::new();
    f.read_to_string(&mut t).expect("Can't convert to a string");
    t
}

/// This function creates a Handlebars instance, applies any helpers to it, then returns the instance
/// Expect helpers within this function to evolve over time as the need for new helpers emerges
///
/// Current helpers:
/// {{hex 16}} will render 0x10
/// {{lower "ABC"}} will render abc
/// {{upper "abc"}} will render ABC
/// {{current_time "%Y-%m-%dT%H:%M:%S"}} will render the current time in the specified format
/// {{toJSON json-content}} will render the JSON representation of json-content
/// {{envVar "ENV_VARIABLE"}} will render the value of the environment variable ENV_VARIABLE
/// {{capitalise "abc def"}} will render Abc Def (i.e. make the first letter in every word a capital letter)
fn register_handlebars() -> Handlebars {

    let mut handlebars = Handlebars::new();

    // register all Handlebars helpers
    handlebars_helper!(hex: |v: i64| format!("0x{:x}", v));
    handlebars_helper!(lower: |s: str| s.to_lowercase());
    handlebars_helper!(upper: |s: str| s.to_uppercase());
    handlebars_helper!(current_time: |fmt: str| format!("{}", Local::now().format(fmt)));
    // handlebars_helper!(toJSON: |json_obj: object| format!("{:#?}", serde_json::to_string_pretty(&json_obj).unwrap()) );
    handlebars_helper!(toJSON: |json_obj_or_none: object|
    if json_obj_or_none.is_empty() {
        "{}".into()
    } else {
        let s = serde_json::to_string_pretty(&json_obj_or_none).unwrap_or_else(|_| "{}".to_string());
        s
    });
    handlebars_helper!(envVar: |s: str| env::var(s).unwrap().to_string());
    handlebars_helper!(capitalise: |s: str| s.to_title_case());

    handlebars.register_helper("hex", Box::new(hex));
    handlebars.register_helper("lower", Box::new(lower));
    handlebars.register_helper("upper", Box::new(upper));
    handlebars.register_helper("current_time", Box::new(current_time));
    handlebars.register_helper("toJSON", Box::new(toJSON));
    handlebars.register_helper("envVar", Box::new(envVar));
    handlebars.register_helper("capitalise", Box::new(capitalise));
    handlebars
}

/// Read from stdin into "pact_str"
/// TODO: Generalise this to read from any file-like interface
fn read_pact_from_stdin() -> Pact {
    let mut pact_str = String::new();
    io::stdin()
        .read_to_string(&mut pact_str)
        .expect("No Pact supplied to stdin");
    let res = serde_json::from_str(&pact_str);
    if res.is_err() {
        eprintln!("{:#?}", res);
        eprintln!("Couldn't parse Pact JSON :-(");
        process::exit(102);
    }
    let pact: Pact = res.unwrap();
    pact
}

/*
fn read_pact(file: Option<File>) -> Pact {
    // Read from stdin into "pact_str"
    let mut pact_str = String::new();
    file
        .read_to_string(&mut pact_str)
        .expect("No Pact supplied to stdin");
    let res = serde_json::from_str(&pact_str);
    if res.is_err() {
        eprintln!("{:#?}", res);
        eprintln!("Couldn't parse Pact JSON :-(");
    }
    let pact: Pact = res.unwrap();
    pact
}
*/
fn main() {
    let pact: Pact = read_pact_from_stdin();
    // let pact: Pact = read_pact(io::stdin());

    eprintln!("The provider is {}", pact.provider.name);

    let template = read_template_file(env::var("TEMPLATE").unwrap());

    let handlebars = register_handlebars();

    let result = handlebars.render_template(&template, &pact);

    // Write template+pact to stdout
    eprintln!("{:#?}", result);
    println!("{}", result.unwrap_or_else(|_| {
        eprintln!("Found a problem");
        process::exit(103)}));
}
