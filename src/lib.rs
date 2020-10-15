mod rjson;

#[cfg(test)]
mod tests {

    use super::rjson::*;

    macro_rules! assert_eq_r {
        ($x:expr, $y:pat) => {
            match $x {
                $y => (),
                _ => panic!("error!!"),
            }
        };
    }

    macro_rules! assert_neq_r {
        ($x:expr, $y:pat) => {
            match $x {
                $y => panic!("error!!"),
                _ => (),
            }
        };
    }

    macro_rules! assert_eq_n_r {
        ($x:expr, $y:expr, $z:expr) => {
            match $x {
                Ok(ParseResult {
                    json,
                    json_struct: JsonStruct::Number(num),
                }) if num == $y && json == $z => (),

                _ => panic!("parse num error!!"),
            }
        };
    }

    macro_rules! assert_eq_n_js {
        ($x:expr, $y:expr) => {
            match $x {
                Ok(JsonStruct::Number(num)) if num == $y => (),
                _ => panic!("parse num error!!"),
            }
        };
    }

    macro_rules! assert_eq_str {
        ($x:expr, $y:expr, $z:expr) => {
            match $x {
                Ok(ParseResult {
                    json,
                    json_struct: JsonStruct::Str(s),
                }) if s == $y && json == $z => (),
                _ => panic!("parse num error!!"),
            }
        };
    }

    macro_rules! assert_eq_str_js {
        ($x:expr, $y:expr) => {
            match $x {
                Ok(JsonStruct::Str(s)) if s == $y => (),
                _ => panic!("parse num error!!"),
            }
        };
    }

    #[test]
    fn test_parse() {
        assert_eq_r!(parse("null"), Ok(JsonStruct::Null));
        assert_eq_r!(parse("false"), Ok(JsonStruct::Boolean(false)));
        assert_neq_r!(parse("false"), Ok(JsonStruct::Boolean(true)));
        assert_eq_r!(parse("true"), Ok(JsonStruct::Boolean(true)));
        assert_neq_r!(parse("true"), Ok(JsonStruct::Boolean(false)));

        assert_eq_r!(parse("tru"), Err(ParseError::ValueError));
        assert_eq_r!(parse("nul"), Err(ParseError::ValueError));
        assert_eq_r!(parse("f"), Err(ParseError::ValueError));

        assert_eq_n_js!(parse("100"), 100.0);

        assert_eq_str_js!(parse("\"hello,world\""), "hello,world");
        assert_eq_str_js!(parse("\"\""), "");
    }

    #[test]
    fn test_white_space_skip() {
        assert_eq_r!(skip_white_space("   "), Ok(""));
        assert_eq_r!(skip_white_space(" 1 "), Ok("1 "));
        assert_eq_r!(skip_white_space("1   "), Ok("1   "));
        assert_eq_r!(skip_white_space("   1"), Ok("1"));
        assert_neq_r!(skip_white_space("   1"), Err(_));
    }

    #[test]
    fn test_null_parse() {
        let null_s = "null";

        println!("null parse: {:?}", parse_for_literal("null", null_s, JsonStruct::Null));
        assert_eq_r!(
            parse_for_literal("null", null_s, JsonStruct::Null),
            Ok(ParseResult {
                json: "",
                json_struct: JsonStruct::Null
            })
        );
        assert_eq_r!(parse_for_literal("   ", null_s, JsonStruct::Null), Err(ParseError::ValueError));
        assert_eq_r!(parse_for_literal("null  ", null_s, JsonStruct::Null), Ok(_));
        assert_eq_r!(parse_for_literal("nul  ", null_s, JsonStruct::Null), Err(_));
        assert_eq_r!(parse_for_literal("nul", null_s, JsonStruct::Null), Err(_));
        assert_neq_r!(
            parse_for_literal("null", null_s, JsonStruct::Null),
            Ok(ParseResult {
                json: "  ",
                json_struct: JsonStruct::Null
            })
        );
        assert_eq_r!(parse_for_literal("  null  ", null_s, JsonStruct::Null), Err(_));
    }

    #[test]
    fn test_false_parse() {
        let false_s = "false";
        assert_eq_r!(
            parse_for_literal("false", false_s, JsonStruct::Boolean(false)),
            Ok(ParseResult {
                json: "",
                json_struct: JsonStruct::Boolean(false)
            })
        );
        assert_eq_r!(parse_for_literal("   ", false_s, JsonStruct::Boolean(false)), Err(ParseError::ValueError));
        assert_eq_r!(
            parse_for_literal("false  ", false_s, JsonStruct::Boolean(false)),
            Ok(ParseResult {
                json: "  ",
                json_struct: JsonStruct::Boolean(false)
            })
        );
        assert_eq_r!(parse_for_literal("fal  ", false_s, JsonStruct::Boolean(false)), Err(_));
        assert_eq_r!(parse_for_literal("fa", false_s, JsonStruct::Boolean(false)), Err(_));
        assert_neq_r!(
            parse_for_literal("false", false_s, JsonStruct::Boolean(false)),
            Ok(ParseResult {
                json: "",
                json_struct: JsonStruct::Boolean(true)
            })
        );
        assert_eq_r!(parse_for_literal("  false  ", false_s, JsonStruct::Boolean(false)), Err(_));
        assert_eq_r!(parse_for_literal("abcd", false_s, JsonStruct::Boolean(false)), Err(_));
    }

    #[test]
    fn test_true_parse() {
        let true_s = "true";

        assert_eq_r!(
            parse_for_literal("true", true_s, JsonStruct::Boolean(true)),
            Ok(ParseResult {
                json: "",
                json_struct: JsonStruct::Boolean(true)
            })
        );
        assert_eq_r!(parse_for_literal("   ", true_s, JsonStruct::Boolean(true)), Err(ParseError::ValueError));
        assert_eq_r!(parse_for_literal("true  ", true_s, JsonStruct::Boolean(true)), Ok(_));
        assert_eq_r!(parse_for_literal("tru  ", true_s, JsonStruct::Boolean(true)), Err(_));
        assert_eq_r!(parse_for_literal("tru", true_s, JsonStruct::Boolean(true)), Err(_));
        assert_neq_r!(
            parse_for_literal("true", true_s, JsonStruct::Boolean(true)),
            Ok(ParseResult {
                json: " ",
                json_struct: JsonStruct::Boolean(true)
            })
        );
        assert_eq_r!(parse_for_literal("  true  ", true_s, JsonStruct::Boolean(true)), Err(_));
        assert_eq_r!(parse_for_literal("abcd", true_s, JsonStruct::Boolean(true)), Err(_));
    }

    #[test]
    fn test_num_parse() {
        assert_eq_n_r!(parse_for_number("10000"), 10000.0, "");

        assert_eq_n_r!(parse_for_number("0.0"), 0.0, "");
        assert_eq_n_r!(parse_for_number("0.0"), -0.0, "");
        assert_eq_n_r!(parse_for_number("0.0"), -0.0, "");
        assert_eq_n_r!(parse_for_number("1.0"), 1.0, "");
        assert_eq_n_r!(parse_for_number("-1.0"), -1.0, "");
        assert_eq_n_r!(parse_for_number("1.5"), 1.5, "");
        assert_eq_n_r!(parse_for_number("-1.5"), -1.5, "");
        assert_eq_n_r!(parse_for_number("3.1416"), 3.1416, "");
        assert_eq_n_r!(parse_for_number("1E10"), 1E10, "");
        assert_eq_n_r!(parse_for_number("1e10"), 1e10, "");
        assert_eq_n_r!(parse_for_number("1E+10"), 1E+10, "");
        assert_eq_n_r!(parse_for_number("1E-10"), 1E-10, "");
        assert_eq_n_r!(parse_for_number("-1E10"), -1E10, "");
        assert_eq_n_r!(parse_for_number("-1e10"), -1e10, "");
        assert_eq_n_r!(parse_for_number("-1E+10"), -1E+10, "");
        assert_eq_n_r!(parse_for_number("-1E-10"), -1E-10, "");
        assert_eq_n_r!(parse_for_number("1.234E+10"), 1.234E+10, "");
        assert_eq_n_r!(parse_for_number("1.234E-10"), 1.234E-10, "");
        assert_eq_n_r!(parse_for_number("0.0"), 1e-10000, "");

        assert_eq_r!(parse_for_number("+0"), Err(ParseError::ValueError));
        assert_neq_r!(parse_for_number("008"), Err(ParseError::ValueError));
        assert_eq_r!(parse_for_number(".123"), Err(ParseError::ValueError));
        assert_neq_r!(parse_for_number("123."), Err(ParseError::ValueError));
    }

    #[test]
    fn test_str_parse() {
        assert_eq_str!(parse_for_string("\"hello,world\""), String::from("hello,world"), "");
        assert_eq_str!(parse_for_string("\"hello,world     \"   "), String::from("hello,world     "), "   ");
        assert_eq_str!(parse_for_string("\"\""), String::from(""), "");
        assert_eq_str!(parse_for_string("\"Hello\\nWorld\""), String::from("Hello\\nWorld"), "");
    }

    #[test]
    fn test_array_parse() {
        let result = parse_for_array("[100,\"hello\",3.14,true,false,10003,[1,2,\"\",3]]");

        assert_eq_r!(result, Ok(ParseResult{json:_, json_struct: JsonStruct::Array(_)}))

    }

    #[test]
    fn test_obj_parse(){
        println!("parse obj: {:?}", parse_for_obj("{\"json\" : 1000}"));
        println!("parse obj: {:?}", parse_for_obj("{\"json\" : 1000, \"gg\": \"gg value\"}"));
        println!("parse obj: {:?}", parse_for_obj("{\"json\" : 1000, \"gg\": \"gg value\", \"subobj\": {\"subobjkkk\": \"subobjvvv\", \"\":[100,\"hello\",3.14,true,false,10003,[1,2,\"\",3]]}}"));
        println!("parse obj: {:?}", parse_for_obj("{\"json\" : 1000, \"gg\": \"gg value\", \"\":[100,\"hello\",3.14,true,false,10003,[1,2,\"\",3]]}"));
    }
}
