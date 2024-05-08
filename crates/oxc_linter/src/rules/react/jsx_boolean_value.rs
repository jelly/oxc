use oxc_ast::{
    ast::{Expression, JSXAttributeItem, JSXAttributeName, JSXAttributeValue},
    AstKind,
};
use oxc_diagnostics::{
    miette::{self, Diagnostic},
    thiserror::{self, Error},
};
use oxc_macros::declare_oxc_lint;
use oxc_span::{CompactStr, Span};

use crate::{context::LintContext, rule::Rule, utils::get_prop_value, AstNode};

#[derive(Debug, Error, Diagnostic)]
#[error(
    "eslint-plugin-react(jsx-boolean-value): Value must be omitted for boolean attribute \"{0}\""
)]
#[diagnostic(severity(warning), help("Value must be omitted for boolean attribute"))]
struct JsxBooleanValueDiagnostic(CompactStr, #[label] Span);

#[derive(Debug, Default, Clone)]
pub struct JsxBooleanValue;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Enforce boolean attributes notation in JSX
    ///
    /// ### Example
    /// ```javascript
    /// const Hello = <Hello personal={true} />;
    /// ```
    JsxBooleanValue,
    correctness, // TODO: change category to `correctness`, `suspicious`, `pedantic`, `perf`, `restriction`, or `style`
             // See <https://oxc-project.github.io/docs/contribute/linter.html#rule-category> for details
);

impl Rule for JsxBooleanValue {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::JSXOpeningElement(jsx_opening_elem) = node.kind() else { return };

        for attr in &jsx_opening_elem.attributes {
            let JSXAttributeItem::Attribute(jsx_attr) = attr else { continue };

            let JSXAttributeName::Identifier(ident) = &jsx_attr.name else { continue };

            match get_prop_value(attr) {
                None => return,
                Some(JSXAttributeValue::ExpressionContainer(container)) => {
                    if let Some(expr) = container.expression.as_expression() {
                        match expr.without_parenthesized() {
                            Expression::BooleanLiteral(expr) => {
                                if expr.value == true {
                                    ctx.diagnostic(JsxBooleanValueDiagnostic(
                                        ident.name.to_compact_str(),
                                        expr.span,
                                    ));
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        ("<App foo />;", Some(serde_json::json!(["never"]))),
        ("<App foo bar={true} />;", Some(serde_json::json!(["always", { "never": ["foo"] }]))),
        ("<App foo />;", None),
        ("<App foo={true} />;", Some(serde_json::json!(["always"]))),
        ("<App foo={true} bar />;", Some(serde_json::json!(["never", { "always": ["foo"] }]))),
        ("<App />;", Some(serde_json::json!(["never", { "assumeUndefinedIsFalse": true }]))),
        (
            "<App foo={false} />;",
            Some(
                serde_json::json!(["never", { "assumeUndefinedIsFalse": true, "always": ["foo"] }]),
            ),
        ),
    ];

    let fail = vec![
        ("<App foo={true} />;", Some(serde_json::json!(["never"]))),
        (
            "<App foo={true} bar={true} baz={true} />;",
            Some(serde_json::json!(["always", { "never": ["foo", "bar"] }])),
        ),
        ("<App foo={true} />;", None),
        ("<App foo = {true} />;", None),
        ("<App foo />;", Some(serde_json::json!(["always"]))),
        ("<App foo bar baz />;", Some(serde_json::json!(["never", { "always": ["foo", "bar"] }]))),
        (
            "<App foo={false} bak={false} />;",
            Some(serde_json::json!(["never", { "assumeUndefinedIsFalse": true }])),
        ),
        (
            "<App foo={true} bar={false} baz={false} bak={false} />;",
            Some(serde_json::json!([
              "always",
              { "assumeUndefinedIsFalse": true, "never": ["baz", "bak"] },
            ])),
        ),
        (
            "<App foo={true} bar={true} baz />;",
            Some(serde_json::json!(["always", { "never": ["foo", "bar"] }])),
        ),
    ];

    Tester::new(JsxBooleanValue::NAME, pass, fail).test_and_snapshot();
}
