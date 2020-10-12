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
                Ok(PareseResult {
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
        assert_eq_r!(
            parse_for_null("null"),
            Ok(PareseResult {
                json: "",
                json_struct: JsonStruct::Null
            })
        );
        assert_eq_r!(parse_for_null("   "), Err(ParseError::ValueError));
        assert_eq_r!(parse_for_null("null  "), Ok(_));
        assert_eq_r!(parse_for_null("nul  "), Err(_));
        assert_eq_r!(parse_for_null("nul"), Err(_));
        assert_neq_r!(
            parse_for_null("null"),
            Ok(PareseResult {
                json: "  ",
                json_struct: JsonStruct::Null
            })
        );
        assert_eq_r!(parse_for_null("  null  "), Err(_));
    }

    #[test]
    fn test_false_parse() {
        assert_eq_r!(
            parse_for_false("false"),
            Ok(PareseResult {
                json: "",
                json_struct: JsonStruct::Boolean(false)
            })
        );
        assert_eq_r!(parse_for_false("   "), Err(ParseError::ValueError));
        assert_eq_r!(
            parse_for_false("false  "),
            Ok(PareseResult {
                json: "  ",
                json_struct: JsonStruct::Boolean(false)
            })
        );
        assert_eq_r!(parse_for_false("fal  "), Err(_));
        assert_eq_r!(parse_for_false("fa"), Err(_));
        assert_neq_r!(
            parse_for_false("false"),
            Ok(PareseResult {
                json: "",
                json_struct: JsonStruct::Boolean(true)
            })
        );
        assert_eq_r!(parse_for_false("  false  "), Err(_));
        assert_eq_r!(parse_for_false("abcd"), Err(_));
    }

    #[test]
    fn test_true_parse() {
        assert_eq_r!(
            parse_for_true("true"),
            Ok(PareseResult {
                json: "",
                json_struct: JsonStruct::Boolean(true)
            })
        );
        assert_eq_r!(parse_for_true("   "), Err(ParseError::ValueError));
        assert_eq_r!(parse_for_true("true  "), Ok(_));
        assert_eq_r!(parse_for_true("tru  "), Err(_));
        assert_eq_r!(parse_for_true("tru"), Err(_));
        assert_neq_r!(
            parse_for_true("true"),
            Ok(PareseResult {
                json: " ",
                json_struct: JsonStruct::Boolean(true)
            })
        );
        assert_eq_r!(parse_for_true("  true  "), Err(_));
        assert_eq_r!(parse_for_true("abcd"), Err(_));
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
    }
}
