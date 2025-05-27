use ruff_python_ast::Expr;
use ruff_text_size::Ranged;

use crate::{Suggest, Suggester};

pub(crate) fn check(suggester: &mut Suggester, expr: &Expr) {
    let Expr::Call(call) = expr else {
        return;
    };

    if let Expr::Name(name) = call.func.as_ref() {
        if name.id.as_str() == "print" {
            suggester.suggests.push(Suggest {
                location: name.range(),
                replacement: "logger.info".into(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Suggest, Suggester, check};
    use indoc::indoc;
    use ruff_python_ast::Expr;
    use ruff_python_parser::parse_expression;

    fn apply_all_suggests(suggester: &Suggester) -> String {
        let mut result = String::with_capacity(suggester.code.len());
        let mut last_end = ruff_text_size::TextSize::new(0);

        let mut suggests: Vec<Suggest> = suggester.suggests.clone();
        suggests.sort_by_key(|suggest| suggest.location.start());

        for suggest in suggester.suggests.clone() {
            result.push_str(
                &suggester.code[last_end.to_usize()..suggest.location.start().to_usize()],
            );
            result.push_str(&suggest.replacement);
            last_end = suggest.location.end();
        }

        result.push_str(&suggester.code[last_end.to_usize()..suggester.code.len()]);

        result
    }

    fn get_expr(expression: &str) -> Expr {
        match parse_expression(&expression) {
            Ok(ast) => ast.into_expr(),
            Err(e) => panic!("Failed to parse code: {e}\n\n{expression}\nis not expression."),
        }
    }

    fn apply(expression: &str) -> String {
        let mut suggester = Suggester::new(expression);
        let expr = get_expr(expression);

        check(&mut suggester, &expr);

        apply_all_suggests(&suggester)
    }

    #[test]
    fn test_simple_print_conversion_ast() {
        let input = indoc! {"
            print(\"Hello, world!\")
        "};
        let expected = indoc! {"
            logger.info(\"Hello, world!\")
        "};

        assert_eq!(apply(input), expected);
    }

    #[test]
    fn test_no_print_statements_ast() {
        let input = indoc! {"
            function()
        "};
        let expected = indoc! {"
            function()
        "};

        assert_eq!(apply(input), expected);
    }

    #[test]
    fn test_variable_named_print_ast() {
        let input = indoc! {"
            print(print)
        "};
        let expected = indoc! {"
            logger.info(print)
        "};

        assert_eq!(apply(input), expected);
    }

    #[test]
    fn test_method_call_named_print_ast() {
        let input = indoc! {"
            obj.print(\"This is a method\")
        "};
        let expected = indoc! {"
            obj.print(\"This is a method\")
        "};

        assert_eq!(apply(input), expected);
    }
}
