use std::collections::HashMap;
use std::str::Chars;

#[derive(Debug)]
pub enum ParseError {
    ValueError,
    LeftQuotationMissing,
    RightQuotationMissing,
    ObjNameError,
    ColonMissing,
    ObjectContentIncomplete,
}

#[derive(Debug)]
pub enum JsonStruct {
    Null,
    Boolean(bool),
    Str(String),
    Number(f32),
    Array(Vec<JsonStruct>),
    Object(HashMap<String, JsonStruct>),
}

#[derive(Debug)]
pub struct ParseResult<'a> {
    pub json: &'a str,
    pub json_struct: JsonStruct,
}

#[derive(Debug)]
pub struct ParseStringResult<'a> {
    pub json: &'a str,
    pub str_value: &'a str,
}

pub fn parse(json: &str) -> Result<JsonStruct, ParseError> {
    return if json.len() <= 0 {
        Ok(JsonStruct::Null)
    } else {
        match parse_value(json) {
            Ok(result) => Ok(result.json_struct),
            Err(e) => Err(e)
        }
    };
}

pub fn parse_value(json: &str) -> Result<ParseResult, ParseError>{
    let result = skip_white_space(json);
    if let Ok(json) = result {
        match &json[..1] {
            "n" => parse_for_literal(json, "null", JsonStruct::Null),
            "f" => parse_for_literal(json, "false", JsonStruct::Boolean(false)),
            "t" => parse_for_literal(json, "true", JsonStruct::Boolean(true)),
            "\"" => parse_for_string(json),
            "[" => parse_for_array(json),
            "{" => parse_for_obj(json),
            _ => parse_for_number(json),
        }
    } else {
        Err(ParseError::ValueError)
    }
}

pub fn parse_for_obj(json: &str) -> Result<ParseResult, ParseError>{
    if &json[..1] == "{" && json.len() > 1 {
        let mut obj_map = HashMap::new();
        let mut json_parse = &json[1..];
        loop {
            let mut obj_key = String::new();
            json_parse = match skip_white_space(&json_parse) {
                Ok(json) if &json[..1] == "\"" => json,
                _ => return Err(ParseError::LeftQuotationMissing)
            };

            json_parse = match parse_for_string_value(&json_parse) {
                Ok(result) => {
                    obj_key.push_str(result.str_value);
                    result.json
                },
                Err(_) => return Err(ParseError::ObjNameError)
            };

            json_parse = match skip_white_space(json_parse) {
                Ok(json) if &json[..1] == ":" => &json[1..],
                _ => return Err(ParseError::ColonMissing)
            };

            json_parse = match parse_value(json_parse) {
                Ok(result) => {
                    obj_map.insert(obj_key, result.json_struct);
                    result.json
                },
                Err(e) => {
                    println!("============");
                    return Err(e);
                }
            };

            if json_parse.len() > 0 {
                if &json_parse[..1] == "}"{
                    return Ok(ParseResult{
                        json: &json_parse[1..],
                        json_struct: JsonStruct::Object(obj_map)
                    })
                } else if &json_parse[..1] == "," {
                    json_parse = &json_parse[1..]
                } else {
                    return Err(ParseError::ObjectContentIncomplete)
                }
            }
        }
    }

    Err(ParseError::ValueError)
}

pub fn parse_for_array(json: &str) -> Result<ParseResult, ParseError>{

    if &json[..1] == "[" && json.len() > 1 {
        let mut v = Vec::new();
        let mut json_parsed = &json[1..];

        loop{
            match &json_parsed[..1] {
                "]" => {
                    return Ok(ParseResult {
                        json: &json_parsed[1..],
                        json_struct: JsonStruct::Array(v),
                    });
                },
                "," => {
                    json_parsed = &json_parsed[1..];
                },
                _ => {
                    match parse_value(&json_parsed[..]) {
                        Ok(ParseResult{
                            json, json_struct,
                        }) => {
                            v.push(json_struct);
                            json_parsed = json;
                        },
                        Err(e) => return Err(e)
                    };
                }
            }
        }

    }

    Err(ParseError::ValueError)
}

pub fn parse_for_string(json: &str) -> Result<ParseResult, ParseError> {
    if &json[..1] == "\"" {
        for (i, &item) in json[1..].as_bytes().iter().enumerate() {
            if item == b'"' {
                return Ok(ParseResult {
                    json: &json[i + 2..json.len()],
                    json_struct: JsonStruct::Str(String::from(&json[1..i + 1])),
                });
            }
        }
    }
    Err(ParseError::ValueError)
}

pub fn parse_for_string_value(json: &str) -> Result<ParseStringResult, ParseError> {
    if &json[..1] == "\"" {
        for (i, &item) in json[1..].as_bytes().iter().enumerate() {
            if item == b'"' {
                return Ok(ParseStringResult {
                    json: &json[i + 2..json.len()],
                    str_value: &json[1..i + 1],
                });
            }
        }
    }
    Err(ParseError::ValueError)
}

pub fn parse_for_literal<'a> (json: & 'a str, literal: &str, json_struct: JsonStruct) -> Result<ParseResult<'a>, ParseError> {
    if json.len() < literal.len() {
        return Err(ParseError::ValueError)
    } else {
        for i in 0..literal.len() {
            if &json[i..i+1] != &literal[i..i+1] {
                return Err(ParseError::ValueError)
            }
        }
        return Ok(ParseResult {
            json: &json[literal.len()..],
            json_struct,
        })
    }
}

pub fn parse_for_number(json: &str) -> Result<ParseResult, ParseError> {
    let mut index:usize = 0;
    let mut chars = json.chars();
    let mut check_c = chars.next();

    if check_c == Some('-') {
        index+=1;
        check_c = chars.next();
    }

    match check_c {
        Some(c) => {
            if c == '0' {
                index+=1;
                check_c = chars.next();
            } else {
        
                if !is_digit_1t9(&c) {
                    return Err(ParseError::ValueError)
                }
                skip_for_num(&mut chars, &mut check_c, &mut index);
            }
        },
        _ => ()
    }

    if check_c == Some('.') {
        index+=1;
        check_c = chars.next();

        match check_c {
            Some(c) => {
                if !is_digit(&c) {
                    return Err(ParseError::ValueError)
                }
                skip_for_num(&mut chars, &mut check_c, &mut index);
            },
            _ => ()
        }

    }

    if check_c == Some('e') || check_c == Some('E') {
        index+=1;
        check_c = chars.next();
        
        if check_c == Some('+') || check_c == Some('-') {
            index+=1;
            check_c = chars.next();
        }

        match check_c {
            Some(c) => {
                if !is_digit(&c) {
                    return Err(ParseError::ValueError)
                }
                skip_for_num(&mut chars, &mut check_c, &mut index);
            },
            _ => ()
        }
    }

    return match &json[..index].parse::<f32>() {
        Ok(num) => Ok(ParseResult {
            json: &json[index..],
            json_struct: JsonStruct::Number(*num),
        }),

        Err(_) => Err(ParseError::ValueError),
    };
} 

fn skip_for_num(chars: &mut Chars, check_c: &mut Option<char>, index: &mut usize){
    while let Some(c) = check_c {
        if is_digit(&c) {
            *index = *index + 1;
            *check_c = chars.next();
        } else {
            break
        }
    }
}

fn is_digit_1t9(d: &char) -> bool {
    *d >= '1' && *d <= '9'
}

fn is_digit(d: &char) -> bool {
    *d >= '0' && *d <= '9'
}

pub fn skip_white_space(json: &str) -> Result<&str, ParseError> {
    for (i, &item) in json.as_bytes().iter().enumerate() {
        if item != b' ' && item != b'\t' && item != b'\n' && item != b'\r' {
            return Ok(&json[i..]);
        }
    }

    return Ok(&json[json.len()..]);
}
