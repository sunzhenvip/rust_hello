use nom::bytes::complete::take_while;
use nom::combinator::opt;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, multispace0},
    combinator::{map, recognize},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};
use std::collections::HashMap;
use std::ffi::NulError;

#[derive(Debug)] // 调试使用
enum JsonValue {
    Null,
    Num(f64),
    Bool(bool),
    Str(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>), // 值是一个元组
}

fn parse_json(s: &str) {
    todo!()
}

fn parse_null(input: &str) -> IResult<&str, JsonValue> {
    map(tag("null"), |_| JsonValue::Null)(input) // 这么写 7.1.3版本可以
}

fn parse_bool(input: &str) -> IResult<&str, JsonValue> {
    alt((
        map(tag("true"), |_| JsonValue::Bool(true)),
        map(tag("false"), |_| JsonValue::Bool(false)),
    ))(input)
}


fn test_null(){
    let input = "null";
    println!("{:?}", parse_null(input));
}

fn main() {
    test_null()
}
