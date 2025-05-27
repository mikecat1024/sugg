use crate::{Suggest, Suggester};

pub(crate) fn apply_suggests(code: &Suggester) -> String {
    let mut result = String::with_capacity(code.code.len());
    let mut last_end = ruff_text_size::TextSize::new(0);

    let mut suggests: Vec<Suggest> = code.suggests.clone();
    suggests.sort_by_key(|suggest| suggest.location.start());

    for suggest in code.suggests.clone() {
        result.push_str(&code.code[last_end.to_usize()..suggest.location.start().to_usize()]);
        result.push_str(&suggest.replacement);
        last_end = suggest.location.end();
    }

    result.push_str(&code.code[last_end.to_usize()..code.code.len()]);

    result
}
