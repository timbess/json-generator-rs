#[macro_use]
extern crate pest_derive;

use exitfailure::ExitFailure;
use pest::iterators::Pair;
use pest::Parser;
use serde_json::Map;
use serde_json::Value;
use std::io;
use std::io::Read;
use std::iter::FromIterator;
use structopt::StructOpt;

#[derive(Parser)]
#[grammar = "expr.pest"]
struct ExprParser;

#[derive(StructOpt)]
#[structopt(about = "Takes in JSON DSL and outputs correctly formatted JSON")]
struct Cli {
    dsl_text: String,
    #[structopt(short = "p", long = "pretty")]
    pretty: bool,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let mut input = String::new();
    let text = match args.dsl_text.as_str() {
        "-" => {
            io::stdin().read_to_string(&mut input)?;
            input.as_str()
        }
        text => text,
    };
    let object: Pair<'_, Rule> = ExprParser::parse(Rule::input, text)?.next().unwrap();

    if args.pretty {
        println!("{:#}", parse(object));
    } else {
        println!("{}", parse(object));
    }
    Ok(())
}

fn parse(pairs: Pair<'_, Rule>) -> Value {
    match pairs.as_rule() {
        Rule::null => Value::Null,
        Rule::bool => Value::Bool(pairs.as_str().parse().unwrap()),
        Rule::number => Value::Number(pairs.as_str().parse().unwrap()),
        Rule::string => Value::String(pairs.as_str().trim_matches('"').to_owned()),
        Rule::array => Value::Array(pairs.into_inner().map(parse).collect()),
        Rule::object => Value::Object(Map::from_iter(
            pairs
                .into_inner()
                .map(|kv_pair| kv_pair.into_inner())
                .map(|mut kv_pair| {
                    (
                        kv_pair.next().unwrap().as_str().to_owned(),
                        parse(kv_pair.next().unwrap()),
                    )
                }),
        )),
        _ => unreachable!("{:?}", pairs),
    }
}
