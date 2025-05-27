use ruff_python_ast::Expr;
use ruff_python_ast::visitor::{Visitor, walk_body};
use ruff_python_parser::{self, parse_module};
use ruff_text_size::TextRange;
use rules::sample;

mod rules;

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct Suggest<'a> {
    location: TextRange,
    replacement: &'a str,
}

struct Suggester<'a> {
    code: &'a str,
    suggests: Vec<Suggest<'a>>,
}

impl<'a> Suggester<'a> {
    fn new(code: &'a str) -> Self {
        Self {
            code,
            suggests: Vec::new(),
        }
    }
}

impl<'a> Visitor<'a> for Suggester<'a> {
    fn visit_expr(&mut self, expr: &'a Expr) {
        sample::check(self, expr);
    }
}

pub fn list_suggests(code: &str) {
    let mut suggester = Suggester::new(code);
    let ast = match parse_module(&suggester.code) {
        Ok(ast_suite) => ast_suite,
        Err(e) => panic!("Failed to parse code: {e}"),
    };

    walk_body(&mut suggester, &ast.syntax().body);
}
