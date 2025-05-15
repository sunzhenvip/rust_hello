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

#[derive(Debug)] // 调试使用
enum JsonValue {
    Null,
    Num(f64),
    Bool(bool),
    Str(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>), // 值是一个元组
}



fn main() {
    println!()
}
