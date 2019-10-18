extern crate serde_json;
extern crate handlebars;
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

#[derive(Serialize,Deserialize,Debug)]
struct Consumer {
    name: String
}

#[derive(Serialize,Deserialize,Debug)]
struct Provider {
    name: String
}

/*
#[derive(Serialize,Deserialize,Debug)]
struct PactVersion {
    pact_version: String
}
*/

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
    //headers: Vec<Header>,
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

    // let pact: serde_json::Value = serde_json::from_str(&pact_str).unwrap();
    // println!("{:?}", serde_json::to_string(&pact).unwrap());
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
    // eprintln!("{}", template_filename);
    //let mut f = File::open(template_filename).unwrap();
    let res2 = File::open(template_filename);
    if !res2.is_ok() {
        eprintln!("{:#?}", res2);
        eprintln!("Template file {} not found", template);
    }
    let mut f = res2.unwrap();

    let mut t = String::new();
    f.read_to_string(&mut t);
    // eprintln!("{}", t);

    let mut handlebars = Handlebars::new();

    /*
    handlebars.register_template_string("toJSON", (Option<serde_json::Value>))
        .to_string().ok().unwrap();
    */

    let result = handlebars.render_template(&t, &pact);

    // Write template+pact to stdout
    // eprintln!("{}", pact_str);
    // eprintln!("{}", t);
    println!("{}", result.unwrap());
}