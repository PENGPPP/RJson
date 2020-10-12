use std::collections::HashMap;

#[derive(Debug)]
pub enum ParseError {
    ValueError,
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
pub struct PareseResult<'a> {
    pub json: &'a str,
    pub json_struct: JsonStruct,
}

pub fn parse(json: &str) -> Result<JsonStruct, ParseError> {
    return if json.len() <= 0 {
        Ok(JsonStruct::Null)
    } else if let Ok(json) = skip_white_space(json) {
        let result = match &json[..1] {
            "n" => parse_for_null(json),
            "f" => parse_for_false(json),
            "t" => parse_for_true(json),
            _ => parse_for_number(json),
        };

        match result {
            Ok(PareseResult { json, json_struct }) => Ok(json_struct),
            Err(e) => Err(e),
        }
    } else {
        Ok(JsonStruct::Null)
    };
}

pub fn parse_for_true(json: &str) -> Result<PareseResult, ParseError> {
    return if json.len() < 4 {
        Err(ParseError::ValueError)
    } else if &json[0..1] == "t" && &json[1..2] == "r" && &json[2..3] == "u" && &json[3..4] == "e" {
        Ok(PareseResult {
            json: &json[4..json.len()],
            json_struct: JsonStruct::Boolean(true),
        })
    } else {
        Err(ParseError::ValueError)
    };
}

pub fn parse_for_false(json: &str) -> Result<PareseResult, ParseError> {
    return if json.len() < 5 {
        Err(ParseError::ValueError)
    } else if &json[0..1] == "f"
        && &json[1..2] == "a"
        && &json[2..3] == "l"
        && &json[3..4] == "s"
        && &json[4..5] == "e"
    {
        Ok(PareseResult {
            json: &json[5..json.len()],
            json_struct: JsonStruct::Boolean(false),
        })
    } else {
        Err(ParseError::ValueError)
    };
}

pub fn parse_for_null(json: &str) -> Result<PareseResult, ParseError> {
    return if json.len() < 4 {
        Err(ParseError::ValueError)
    } else if &json[0..1] == "n" && &json[1..2] == "u" && &json[2..3] == "l" && &json[3..4] == "l" {
        Ok(PareseResult {
            json: &json[4..json.len()],
            json_struct: JsonStruct::Null,
        })
    } else {
        Err(ParseError::ValueError)
    };
}

pub fn parse_for_number(json: &str) -> Result<PareseResult, ParseError> {
    for (i, &item) in json.as_bytes().iter().enumerate() {
        let end = if item == b' ' || item == b'\t' || item == b'\n' || item == b'\r' {
            i
        } else if i == (json.len() - 1) {
            json.len()
        } else {
            continue;
        };

        return match &json[..end].parse::<f32>() {
            Ok(num) => Ok(PareseResult {
                json: &json[end..],
                json_struct: JsonStruct::Number(*num),
            }),

            Err(_) => Err(ParseError::ValueError),
        };
    }
    return Err(ParseError::ValueError);
}

pub fn skip_white_space(json: &str) -> Result<&str, ParseError> {
    for (i, &item) in json.as_bytes().iter().enumerate() {
        if item != b' ' && item != b'\t' && item != b'\n' && item != b'\r' {
            return Ok(&json[i..]);
        }
    }

    return Ok(&json[json.len()..]);
}
