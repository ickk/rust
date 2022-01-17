//! Completes identifiers in format string literals.

use ide_db::helpers::format_string::is_format_string;
use itertools::Itertools;
use syntax::{ast, AstToken, TextRange, TextSize};

use crate::{context::CompletionContext, CompletionItem, CompletionItemKind, Completions};

/// Complete identifiers in format strings.
pub(crate) fn format_string(acc: &mut Completions, ctx: &CompletionContext) {
    if true {
        return;
    }
    let string = match ast::String::cast(ctx.token.clone()) {
        Some(it) if is_format_string(&it) => it,
        _ => return,
    };
    let cursor = ctx.position.offset;
    let lit_start = ctx.token.text_range().start();
    let cursor_in_lit = cursor - lit_start;

    let prefix = &string.text()[..cursor_in_lit.into()];
    let braces = prefix.char_indices().rev().skip_while(|&(_, c)| c.is_alphanumeric()).next_tuple();
    let brace_offset = match braces {
        // escaped brace
        Some(((_, '{'), (_, '{'))) => return,
        Some(((idx, '{'), _)) => lit_start + TextSize::from(idx as u32 + 1),
        _ => return,
    };

    let source_range = TextRange::new(brace_offset, cursor);
    ctx.locals.iter().for_each(|(name, _)| {
        CompletionItem::new(CompletionItemKind::Binding, source_range, name.to_smol_str())
            .add_to(acc);
    })
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use crate::tests::completion_list_no_kw;

    fn check(ra_fixture: &str, expect: Expect) {
        let actual = completion_list_no_kw(ra_fixture);
        expect.assert_eq(&actual);
    }

    #[test]
    fn no_completion_without_brace() {
        check(
            r#"
macro_rules! format_args {
($lit:literal $(tt:tt)*) => { 0 },
}
fn main() {
let foobar = 1;
format_args!("f$0");
}
"#,
            expect![[]],
        );
    }

    //     #[test]
    //     fn completes_locals() {
    //         check_edit(
    //             "foobar",
    //             r#"
    // macro_rules! format_args {
    //     ($lit:literal $(tt:tt)*) => { 0 },
    // }
    // fn main() {
    //     let foobar = 1;
    //     format_args!("{f$0");
    // }
    // "#,
    //             r#"
    // macro_rules! format_args {
    //     ($lit:literal $(tt:tt)*) => { 0 },
    // }
    // fn main() {
    //     let foobar = 1;
    //     format_args!("{foobar");
    // }
    // "#,
    //         );
    //         check_edit(
    //             "foobar",
    //             r#"
    // macro_rules! format_args {
    //     ($lit:literal $(tt:tt)*) => { 0 },
    // }
    // fn main() {
    //     let foobar = 1;
    //     format_args!("{$0");
    // }
    // "#,
    //             r#"
    // macro_rules! format_args {
    //     ($lit:literal $(tt:tt)*) => { 0 },
    // }
    // fn main() {
    //     let foobar = 1;
    //     format_args!("{foobar");
    // }
    // "#,
    //         );
    //     }
}
