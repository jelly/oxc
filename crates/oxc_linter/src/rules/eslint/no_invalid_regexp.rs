use oxc_ast::{ast::Argument, AstKind};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{context::LintContext, rule::Rule, AstNode};

#[derive(Debug, Default, Clone)]
pub struct NoInvalidRegexp;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallow invalid regular expression strings in RegExp constructors
    ///
    /// ### Why is this bad?
    ///
    /// An invalid pattern in a regular expression literal is a SyntaxError when the code is
    /// parsed, but an invalid string in RegExp constructors throws a SyntaxError only when the
    /// code is executed.
    ///
    /// ### Example
    /// ```javascript
    /// ```
    NoInvalidRegexp,
    nursery, // TODO: change category to `correctness`, `suspicious`, `pedantic`, `perf`, `restriction`, or `style`
             // See <https://oxc.rs/docs/contribute/linter.html#rule-category> for details
);

impl Rule for NoInvalidRegexp {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        match node.kind() {
            AstKind::NewExpression(new_expr) => {
                if new_expr
                    .callee
                    .without_parenthesized()
                    .get_identifier_reference()
                    .is_some_and(|x| x.name == "RegExp")
                {
                    dbg!(&new_expr);
                    check_arguments(&new_expr.arguments);
                }
            }
            AstKind::CallExpression(call_expr) => {
                if call_expr
                    .callee
                    .without_parenthesized()
                    .get_identifier_reference()
                    .is_some_and(|x| x.name == "RegExp")
                {
                    dbg!(&call_expr);
                    check_arguments(&call_expr.arguments);
                }
            }
            _ => {}
        }
    }
}

// Should return arguments I suppose
fn check_arguments(args: &Vec<'_, Argument<'_>>) -> bool {
    dbg!(&args);
    if let Some(expr) = args.get(1).and_then(Argument::as_expression) {
        if !expr.is_string_literal() {
            return false; // skip on indeterminate flag, e.g. RegExp('a  b', flags)
        }
    }

    true
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        ("RegExp('')", None),
        ("RegExp()", None),
        ("RegExp('.', 'g')", None),
        ("new RegExp('.')", None),
        ("new RegExp", None),
        ("new RegExp('.', 'im')", None),
        ("global.RegExp('\\')", None),
        ("new RegExp('.', y)", None),
        ("new RegExp('.', 'y')", None),
        ("new RegExp('.', 'u')", None),
        ("new RegExp('.', 'yu')", None),
        ("new RegExp('/', 'yu')", None),
        ("new RegExp('\\/', 'yu')", None),
        ("new RegExp('\\u{65}', 'u')", None),
        ("new RegExp('\\u{65}*', 'u')", None),
        ("new RegExp('[\\u{0}-\\u{1F}]', 'u')", None),
        ("new RegExp('.', 's')", None),
        ("new RegExp('(?<=a)b')", None),
        ("new RegExp('(?<!a)b')", None),
        ("new RegExp('(?<a>b)\\k<a>')", None),
        ("new RegExp('(?<a>b)\\k<a>', 'u')", None),
        ("new RegExp('\\p{Letter}', 'u')", None),
        ("RegExp('{', flags)", None),
        ("new RegExp('{', flags)", None),
        ("RegExp('\\u{0}*', flags)", None),
        ("new RegExp('\\u{0}*', flags)", None),
        ("RegExp('{', flags)", Some(serde_json::json!([{ "allowConstructorFlags": ["u"] }]))),
        ("RegExp('\\u{0}*', flags)", Some(serde_json::json!([{ "allowConstructorFlags": ["a"] }]))),
        ("new RegExp(pattern, 'g')", None),
        ("new RegExp('.' + '', 'g')", None),
        ("new RegExp(pattern, '')", None),
        ("new RegExp(pattern)", None),
        ("new RegExp('(?<\\ud835\\udc9c>.)', 'g')", None),
        ("new RegExp('(?<\\u{1d49c}>.)', 'g')", None),
        ("new RegExp('(?<𝒜>.)', 'g');", None),
        ("new RegExp('\\p{Script=Nandinagari}', 'u');", None),
        ("new RegExp('a+(?<Z>z)?', 'd')", None),
        ("new RegExp('\\p{Script=Cpmn}', 'u')", None),
        ("new RegExp('\\p{Script=Cypro_Minoan}', 'u')", None),
        ("new RegExp('\\p{Script=Old_Uyghur}', 'u')", None),
        ("new RegExp('\\p{Script=Ougr}', 'u')", None),
        ("new RegExp('\\p{Script=Tangsa}', 'u')", None),
        ("new RegExp('\\p{Script=Tnsa}', 'u')", None),
        ("new RegExp('\\p{Script=Toto}', 'u')", None),
        ("new RegExp('\\p{Script=Vith}', 'u')", None),
        ("new RegExp('\\p{Script=Vithkuqi}', 'u')", None),
        ("new RegExp('[A--B]', 'v')", None),
        ("new RegExp('[A&&B]', 'v')", None),
        ("new RegExp('[A--[0-9]]', 'v')", None),
        ("new RegExp('[\\p{Basic_Emoji}--\\q{a|bc|def}]', 'v')", None),
        ("new RegExp('[A--B]', flags)", None),
        ("new RegExp('[[]\\u{0}*', flags)", None),
        ("new RegExp('.', 'g')", Some(serde_json::json!([{ "allowConstructorFlags": [] }]))),
        ("new RegExp('.', 'g')", Some(serde_json::json!([{ "allowConstructorFlags": ["a"] }]))),
        ("new RegExp('.', 'a')", Some(serde_json::json!([{ "allowConstructorFlags": ["a"] }]))),
        ("new RegExp('.', 'ag')", Some(serde_json::json!([{ "allowConstructorFlags": ["a"] }]))),
        ("new RegExp('.', 'ga')", Some(serde_json::json!([{ "allowConstructorFlags": ["a"] }]))),
        (
            "new RegExp(pattern, 'ga')",
            Some(serde_json::json!([{ "allowConstructorFlags": ["a"] }])),
        ),
        (
            "new RegExp('.' + '', 'ga')",
            Some(serde_json::json!([{ "allowConstructorFlags": ["a"] }])),
        ),
        (
            "new RegExp('.', 'a')",
            Some(serde_json::json!([{ "allowConstructorFlags": ["a", "z"] }])),
        ),
        (
            "new RegExp('.', 'z')",
            Some(serde_json::json!([{ "allowConstructorFlags": ["a", "z"] }])),
        ),
        (
            "new RegExp('.', 'az')",
            Some(serde_json::json!([{ "allowConstructorFlags": ["a", "z"] }])),
        ),
        (
            "new RegExp('.', 'za')",
            Some(serde_json::json!([{ "allowConstructorFlags": ["a", "z"] }])),
        ),
        (
            "new RegExp('.', 'agz')",
            Some(serde_json::json!([{ "allowConstructorFlags": ["a", "z"] }])),
        ),
    ];

    let fail = vec![
        ("RegExp('[');", None),
        ("RegExp('.', 'z');", None),
        ("RegExp('.', 'a');", Some(serde_json::json!([{}]))),
        ("new RegExp('.', 'a');", Some(serde_json::json!([{ "allowConstructorFlags": [] }]))),
        ("new RegExp('.', 'z');", Some(serde_json::json!([{ "allowConstructorFlags": ["a"] }]))),
        ("RegExp('.', 'a');", Some(serde_json::json!([{ "allowConstructorFlags": ["A"] }]))),
        ("RegExp('.', 'A');", Some(serde_json::json!([{ "allowConstructorFlags": ["a"] }]))),
        ("new RegExp('.', 'az');", Some(serde_json::json!([{ "allowConstructorFlags": ["z"] }]))),
        ("new RegExp(')');", None),
        ("new RegExp('\\a', 'u');", None),
        ("new RegExp('\\a', 'u');", Some(serde_json::json!([{ "allowConstructorFlags": ["u"] }]))),
        ("RegExp('\u{0}*');", None),
        ("new RegExp('\u{0}*');", None),
        ("new RegExp('\u{0}*', '');", None),
        (
            "new RegExp('\u{0}*', 'a');",
            Some(serde_json::json!([{ "allowConstructorFlags": ["a"] }])),
        ),
        ("RegExp('\u{0}*');", Some(serde_json::json!([{ "allowConstructorFlags": ["a"] }]))),
        ("new RegExp('\');", None),
        ("RegExp(')' + '', 'a');", None),
        (
            "new RegExp('.' + '', 'az');",
            Some(serde_json::json!([{ "allowConstructorFlags": ["z"] }])),
        ),
        (
            "new RegExp(pattern, 'az');",
            Some(serde_json::json!([{ "allowConstructorFlags": ["a"] }])),
        ),
        ("new RegExp('[[]', 'v');", None),
        ("new RegExp('.', 'uv');", None),
        ("new RegExp(pattern, 'uv');", None),
        ("new RegExp('[A--B]' /* valid only with `v` flag */, 'u')", None),
        ("new RegExp('[[]\\u{0}*' /* valid only with `u` flag */, 'v')", None),
    ];

    Tester::new(NoInvalidRegexp::NAME, pass, fail).test_and_snapshot();
}
