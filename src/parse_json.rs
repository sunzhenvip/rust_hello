#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数
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

fn parse_num(input: &str) -> IResult<&str, JsonValue> {
    // digit1 表示数字 pair 要求匹配魔 opt 可选
    map(recognize(pair(opt(char('-')), digit1)), |s: &str| {
        JsonValue::Num(s.parse().unwrap())
    })(input)
}

fn parse_str(input: &str) -> IResult<&str, JsonValue> {
    map(
        delimited(
            char('"'), // 是不是有双引号开头结尾也是双引号 有可能是字符串类型
            take_while(|c| c != '"'),
            char('"'),
        ),
        |s: &str| JsonValue::Str(s.to_string()),
    )(input)
}

fn parse_value(input: &str) -> IResult<&str, JsonValue> {
    // 依次进行匹配
    // "        " 中间可能有空格 前面的空格干掉
    preceded(
        multispace0,
        alt((parse_null, parse_num, parse_bool, parse_str, parse_array)),
    )(input)
}

fn parse_array(input: &str) -> IResult<&str, JsonValue> {
    map(
        delimited(
            char('['),
            separated_list0(
                preceded(multispace0, char(',')),
                preceded(multispace0, parse_value),
            ),
            preceded(multispace0, char(']')),
        ),
        JsonValue::Array
    )(input)
}

fn test_null() {
    let input = "null";
    println!("{:?}", parse_null(input));
}

fn test_num() {
    let input = "-848";
    println!("{:?}", parse_num(input));

    let input = "郑州";
    println!("{:?}", parse_num(input));
}

fn test_str() {
    println!("{:?}", parse_str(r#""hello""#));
}

fn test_array() {
    println!("{:?}", parse_array(r#"[1,2,3]"#));
}

fn main() {
    test_null();
    test_num();
    test_str();
    test_array();
}
