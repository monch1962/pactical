use handlebars::{Handlebars, HelperResult, Output, RenderContext, Context, Helper};
use inflector::Inflector;
use rand::Rng;
use lipsum::{lipsum, lipsum_title};
use std::convert::TryInto;
use std::env;
use chrono::Local;

/// Return a random integer, between 'min' and 'max'
fn random_int(min: u64, max: u64) -> String {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(min, max);
    eprintln!("Integer: {}", r);
    // println!("Float: {}", rng.gen_range(0.0, 10.0));
    r.to_string()
}

/// Return a random decimal number, of length 'digits'
fn random_decimal(digits: u64) -> String {
    const CHARSET: &[u8] = b"0123456789";
    let mut rng = rand::thread_rng();

    let r: String = (0..digits)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    eprintln!("{:?}", r);
    r
}

/// Return a random hexadecimal number, of length 'digits'
fn random_hexadecimal(digits: u64) -> String {
    const CHARSET: &[u8] = b"0123456789abcdef";
    let mut rng = rand::thread_rng();

    let r: String = (0..digits)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    eprintln!("{:?}", r);
    r
}

/// Return a random string that conforms to the supplied regex pattern
fn random_regex(pattern: String) -> String {
    let utf8_hir = regex_syntax::ParserBuilder::new()
        .unicode(false)
        .allow_invalid_utf8(true)
        .build()
        .parse(&pattern)
        .unwrap();
    let utf8_gen = rand_regex::Regex::with_hir(utf8_hir, 100).unwrap();
    format!("{:?}", utf8_gen)
}

/// Return a random UIID
fn random_uuid() -> String {
    format!(
        "{}-{}-{}-{}-{}",
        random_hexadecimal(8),
        random_hexadecimal(4),
        random_hexadecimal(4),
        random_hexadecimal(4),
        random_hexadecimal(12)
    )
}

/// Return a random string, of size 'size'
fn random_string(size: u64) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();

    let r: String = (0..size)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    eprintln!("{:?}", r);
    r
}

/// Return a random boolean
fn random_boolean() -> String {
    let mut rng = rand::thread_rng();
    let r = rng.gen::<bool>();
    eprintln!("Boolean: {}", r);
    r.to_string()
}

fn random_boolean_helper(
    _h: &Helper,
    _: &Handlebars,
    _: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let result = random_boolean().to_string();
    out.write(result.as_ref())?;
    Ok(())
}

fn lorum_text_helper(words: i64) -> String {
    lipsum(words.try_into().unwrap())
}

fn lorum_title_helper(
    _h: &Helper,
    _: &Handlebars,
    _: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let result = lipsum_title().to_string();
    out.write(result.as_ref())?;
    Ok(())
}

fn random_uuid_helper(
    _h: &Helper,
    _: &Handlebars,
    _: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let result = random_uuid().to_string();
    out.write(result.as_ref())?;
    Ok(())
}

/*
fn random_string_helper(h: &Helper, _: &Handlebars, _: &Context, rc: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    let param = h.param(0).unwrap();
    let result = format!("{}", random_string(*param.value().render().as_ref()));
    out.write(result.as_ref())?;
    Ok(())
}
*/

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
pub fn register_handlebars() -> Handlebars {
    let mut handlebars = Handlebars::new();

    // register all Handlebars helpers
    handlebars_helper!(hex: |v: i64| format!("0x{:x}", v));
    handlebars_helper!(lower: |s: str| s.to_lowercase());
    handlebars_helper!(upper: |s: str| s.to_uppercase());
    handlebars_helper!(current_time: |fmt: str| format!("{}", Local::now().format(fmt)));
    handlebars_helper!(rand_decimal: |digits: u64| random_decimal(digits));
    handlebars_helper!(rand_int: |min: u64, max: u64| random_int(min, max));
    handlebars_helper!(rand_hexadecimal: |num_digits: u64| random_hexadecimal(num_digits));
    handlebars_helper!(rand_regex: |r: str| random_regex(r.to_string()));
    handlebars_helper!(rand_string: |chars: u64| random_string(chars).to_string());

    handlebars_helper!(toJSON: |json_obj_or_none: object|
    if json_obj_or_none.is_empty() {
        "{}".into()
    } else {
        serde_json::to_string_pretty(&json_obj_or_none).unwrap_or_else(|_| "{}".to_string())
    });
    handlebars_helper!(escapedJSON: |json_obj_or_none: object| format!("{:?}", serde_json::to_string(&json_obj_or_none).unwrap()));
    handlebars_helper!(envVar: |s: str| env::var(s).unwrap().to_string());
    handlebars_helper!(capitalise: |s: str| s.to_title_case());
    handlebars_helper!(lorum_text: |words: i64| lorum_text_helper(words));

    handlebars.register_helper("hex", Box::new(hex));
    handlebars.register_helper("lower", Box::new(lower));
    handlebars.register_helper("upper", Box::new(upper));
    handlebars.register_helper("current_time", Box::new(current_time));
    handlebars.register_helper("random_decimal", Box::new(rand_decimal));
    handlebars.register_helper("random_integer", Box::new(rand_int));
    handlebars.register_helper("random_hexadecimal", Box::new(rand_hexadecimal));
    handlebars.register_helper("random_regex", Box::new(rand_regex));
    handlebars.register_helper("random_uuid", Box::new(random_uuid_helper));
    handlebars.register_helper("random_string", Box::new(rand_string));
    handlebars.register_helper("random_boolean", Box::new(random_boolean_helper));
    handlebars.register_helper("toJSON", Box::new(toJSON));
    handlebars.register_helper("escapedJSON", Box::new(escapedJSON));
    handlebars.register_helper("envVar", Box::new(envVar));
    handlebars.register_helper("capitalise", Box::new(capitalise));
    handlebars.register_helper("lorum_text", Box::new(lorum_text));
    handlebars.register_helper("lorum_title", Box::new(lorum_title_helper));
    handlebars
}

#[cfg(test)]
mod unittests {
    // use super::*;
    use handlebars_helpers;

    #[test]
    fn random_boolean_working() {
        let r:String = handlebars_helpers::random_boolean();
        assert!(r == "true" || r == "false");
    }

    #[test]
    fn random_string_working() {
        use regex::Regex;
        let re = Regex::new(r"^.{12}$").unwrap();
        let r:String = handlebars_helpers::random_string(12);
        assert!(re.is_match(&r));
    }

    #[test]
    fn random_uuid_working() {
        use regex::Regex;
        let re =
            Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();
        let r:String = handlebars_helpers::random_uuid();
        assert!(re.is_match(&r));
    }

    #[test]
    fn random_hexadecimal_working() {
        use regex::Regex;
        let re = Regex::new(r"^[0-9a-f]{4}$").unwrap();
        let r:String = handlebars_helpers::random_hexadecimal(4);
        assert!(re.is_match(&r));
    }

    #[test]
    fn random_decimal_working() {
        let r = handlebars_helpers::random_decimal(5);
        assert!(r.parse::<u64>().unwrap() < 99999);
    }

    #[test]
    fn random_int_working() {
        const MIN: u64 = 47;
        const MAX: u64 = 193;
        let r = handlebars_helpers::random_int(MIN, MAX);
        assert!(r.parse::<u64>().unwrap() >= MIN);
        assert!(r.parse::<u64>().unwrap() < MAX);
    }
}
