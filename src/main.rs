extern crate serde_json;
#[macro_use] extern crate handlebars;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use handlebars::Handlebars;
// use serde_json::json;
// use std::fs;
// use std::io::{self, Read};
use std::env;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

// extern crate serialize;
// use serialize::json;

extern crate chrono;
use chrono::Local;


#[derive(Serialize,Deserialize,Debug)]
struct Consumer {
    name: String
}

#[derive(Serialize,Deserialize,Debug)]
struct Provider {
    name: String
}

#[derive(Serialize,Deserialize,Debug)]
struct PactSpecification {
    version: String
}

#[derive(Serialize,Deserialize,Debug)]
struct Metadata {
    pactSpecification: PactSpecification
}

#[derive(Serialize,Deserialize,Debug)]
struct Header {
    key: String,
    value: String
}

#[derive(Serialize,Deserialize,Debug)]
struct HeaderString {
    header: String
}

#[derive(Serialize,Deserialize,Debug)]
struct Request {
    method: String,
    path: String,
    headers: Option<serde_json::Value>,
    query: Option<serde_json::Value>,
    body: Option<serde_json::Value>,
    matching_rules: Option<serde_json::Value>
}

#[derive(Serialize,Deserialize,Debug)]
struct Response {
    status: Option<u16>,
    headers: Option<serde_json::Value>,
    body: Option<serde_json::Value>,
    generators: Option<serde_json::Value>,
    matchingRules: Option<serde_json::Value>
}

#[derive(Serialize,Deserialize,Debug)]
struct Interaction {
    description: String,
    providerState: Option<serde_json::Value>,
    request: Request,
    response: Response
}

#[derive(Serialize,Deserialize,Debug)]
struct Pact {
    consumer: Consumer,
    provider: Provider,
    interactions: Vec<Interaction>,
    metadata: Metadata
}

/*
fn read_template_file(template_env_var: String) -> String {
    let template = template_env_var.unwrap();
    let template_filename = format!("./templates/{}.hbs", template);
    let res = File::open(template_filename);
    if !res.is_ok() {
        eprintln!("{:#?}", res);
        eprintln!("Template file {} not found", template_filename);
    }
    let template_content = res.unwrap();
    let mut t = String::new();
    template_content.read_to_string(&mut t);
    t
}
*/

fn main() {
    // Read from stdin into "pact_str"
    let mut pact_str = String::new();
    io::stdin().read_to_string(&mut pact_str).expect("No Pact supplied to stdin");
    let res = serde_json::from_str(&pact_str);
    if !res.is_ok() {
        eprintln!("{:#?}", res);
        eprintln!("Couldn't parse Pact JSON :-(");
    }
    let pact: Pact = res.unwrap();
    eprintln!("The provider is {}", pact.provider.name);
    

    // Read template from environment variable
    let template = env::var("TEMPLATE").unwrap();

    let template_filename = format!("./templates/{}.hbs", template);
    let res2 = File::open(template_filename);
    if !res2.is_ok() {
        eprintln!("{:#?}", res2);
        eprintln!("Template file {} not found", template);
    }
    let mut f = res2.unwrap();

    let mut t = String::new();
    f.read_to_string(&mut t);

    let mut handlebars = Handlebars::new();

    // register all Handlebars helpers
    handlebars_helper!(hex: |v: i64| format!("0x{:x}", v));
    handlebars_helper!(lower: |s: str| s.to_lowercase());
    handlebars_helper!(upper: |s: str| s.to_uppercase());
    handlebars_helper!(current_time: |fmt: str| format!("{}", Local::now().format(fmt)));
    handlebars_helper!(toJSON: |json_str: str| /*format!("{}", json_str)*/ json_str.to_string());

    handlebars.register_helper("hex", Box::new(hex));
    handlebars.register_helper("lower", Box::new(lower));
    handlebars.register_helper("upper", Box::new(upper));   
    handlebars.register_helper("current_time", Box::new(current_time));
    handlebars.register_helper("toJSON", Box::new(toJSON));

    let result = handlebars.render_template(&t, &pact);

    // Write template+pact to stdout
    println!("{}", result.unwrap());
}
