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

mod pact;
use pact::Pact;

mod handlebars_helpers; 

extern crate serde_json;
#[macro_use]
extern crate handlebars;
extern crate serde;

extern crate lipsum;

extern crate serde_derive;

use std::{env, fs};

// use serde::{Deserialize, Serialize};
// use serde_json::Value as JsonValue;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process;

extern crate chrono;

extern crate inflector;

extern crate rand;

extern crate rand_regex;
extern crate regex;
extern crate regex_syntax;

/// Read template file from the specified environment variable
/// and return the template as a string
fn read_template_file(template_env_var: String) -> String {
    let template = template_env_var;
    eprintln!("Template env var: {}", template);

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

/// read_pact reads a Pact contract from a filename
/// and returns the Pact as a Pact
fn read_pact(filename: String) -> Pact {
    let pact_str = fs::read_to_string(filename).expect("No Pact supplied in filename");
    let res = serde_json::from_str(&pact_str);
    if res.is_err() {
        eprintln!("{:#?}", res);
        eprintln!("Couldn't parse Pact JSON :-(");
    }
    let pact: Pact = res.unwrap();
    pact
}

fn main() {
    let pact: Pact = read_pact_from_stdin();
    // let pact: Pact = read_pact(Stdin);

    eprintln!("The provider is {}", pact.provider.name);

    let template = read_template_file(env::var("TEMPLATE").unwrap());

    let handlebars = handlebars_helpers::register_handlebars();

    let result = handlebars.render_template(&template, &pact);

    // Write template+pact to stdout
    eprintln!("{:#?}", result);
    println!(
        "{}",
        result.unwrap_or_else(|_| {
            eprintln!("Found a problem");
            process::exit(103)
        })
    );
}
