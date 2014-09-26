#![crate_name = "posgen"]
#![feature(macro_rules, quote, phase)]

#[phase(plugin)]
extern crate phf_mac;
extern crate phf;
extern crate serialize;
extern crate libc;
extern crate toml;

use std::{os};
use std::io::{fs};

mod il;
mod clang;
mod gen;
mod parser;
mod types;

struct Const {
    name: String,
    c_type: String,
    rs_type: String,
}

struct Defs {
    header: String,
    types: Vec<String>,
    consts: Vec<Const>,
}

fn load_defs(file: &[u8]) -> Option<Defs> {
    #[deriving(Decodable)]
    struct Custom {
        c_type: String,
        rs_type: String,
        vars: Vec<String>,
    }
    #[deriving(Decodable)]
    struct Defs_ {
        header:     String,
        types:      Option<Vec<String>>,
        chars:      Option<Vec<String>>,
        schars:     Option<Vec<String>>,
        uchars:     Option<Vec<String>>,
        shorts:     Option<Vec<String>>,
        ushorts:    Option<Vec<String>>,
        ints:       Option<Vec<String>>,
        uints:      Option<Vec<String>>,
        longs:      Option<Vec<String>>,
        ulongs:     Option<Vec<String>>,
        longlongs:  Option<Vec<String>>,
        ulonglongs: Option<Vec<String>>,
        floats:     Option<Vec<String>>,
        doubles:    Option<Vec<String>>,
        custom:     Option<Vec<Custom>>,
    }
    let mut defs = fs::File::open(&Path::new(file))
                                   .and_then(|mut f| f.read_to_end()).ok()
                                   .and_then(|b| String::from_utf8(b).ok())
                                   .and_then(|s| toml::decode_str(s.as_slice()));
    let mut defs: Defs_ = match defs {
        Some(..) => defs.take().unwrap(),
        _ => return None,
    };
    let mut consts = Vec::new();
    macro_rules! decode {
        ($ty:ident, $ct:expr, $rt:expr) => {
            if defs.$ty.is_some() {
                for v in defs.$ty.take().unwrap().into_iter() {
                    consts.push( Const { name: v,
                                         c_type: $ct.to_string(),
                                         rs_type: $rt.to_string(), });
                }
            }
        }
    };
    decode!(chars,      "char",               "::char_t");
    decode!(schars,     "signed char",        "::schar_t");
    decode!(uchars,     "unsigned char",      "::uchar_t");
    decode!(shorts,     "short",              "::short_t");
    decode!(ushorts,    "unsigned short",     "::ushort_t");
    decode!(ints,       "int",                "::int_t");
    decode!(uints,      "unsigned int",       "::uint_t");
    decode!(longs,      "long",               "::long_t");
    decode!(ulongs,     "unsigned long",      "::ulong_t");
    decode!(longlongs,  "long long",          "::longlong_t");
    decode!(ulonglongs, "unsigned long long", "::ulonglong_t");
    decode!(floats,     "float",              "::float_t");
    decode!(doubles,    "double",             "::double_t");
    if defs.custom.is_some() {
        for v in defs.custom.take().unwrap().into_iter() {
            let Custom { vars, c_type, rs_type } = v;
            for var in vars.into_iter() {
                consts.push( Const { name: var,
                                     c_type: c_type.clone(),
                                     rs_type: rs_type.clone(), });
            }
        }
    }
    let defs = Defs {
        header: defs.header.clone(),
        types: defs.types.take().unwrap_or(Vec::new()),
        consts: consts,
    };
    Some(defs)
}

macro_rules! errln {
    ($($arg:tt)*) => {(writeln!(::std::io::stdio::stderr(), $($arg)*)).ok()}
}

fn main() {
    let bind_args = os::args_as_bytes();
    if bind_args.len() != 2 {
        errln!("USAGE: posgen def.toml");
        os::set_exit_status(1);
        return;
    }
    load_defs(bind_args[1].as_slice())
        .and_then(|d| parser::parse(&d).ok())
        .map(|g| gen::gen(g));
}
