use oxc_ast::{
    ast::{JSXAttributeValue, JSXElementName, JSXExpression},
    AstKind,
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;
use phf::phf_set;

use crate::{
    context::LintContext,
    rule::Rule,
    utils::{get_element_type, get_prop_value, has_jsx_prop_lowercase},
    AstNode,
};

#[derive(Debug, Default, Clone)]
pub struct IframeMissingSandbox;

declare_oxc_lint!(
    /// ### What it does
    ///
    ///
    /// ### Why is this bad?
    ///
    ///
    /// ### Example
    /// ```javascript
    /// var React = require('react');
    ///
    /// var Frame = () => (
    /// <div>
    /// <iframe></iframe>
    ///   {React.createElement('iframe')}
    /// </div>
    /// );
    /// ```
    IframeMissingSandbox,
    correctness, // TODO: change category to `correctness`, `suspicious`, `pedantic`, `perf`, `restriction`, or `style`
             // See <https://oxc.rs/docs/contribute/linter.html#rule-category> for details
);

fn iframe_missing_sandbox_diagnostic(span0: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("eslint-plugin-react(iframe-missing-sandbox): Missing `sandbox` attribute for the `iframe` element.")
        .with_help("Provide sandbox property for iframe element.")
        .with_labels([span0.into()])
}

const ALLOWED_VALUES: phf::Set<&'static str> = phf_set! {
  // From https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe#attr-sandbox
  "",
  "allow-downloads-without-user-activation",
  "allow-downloads",
  "allow-forms",
  "allow-modals",
  "allow-orientation-lock",
  "allow-pointer-lock",
  "allow-popups",
  "allow-popups-to-escape-sandbox",
  "allow-presentation",
  "allow-same-origin",
  "allow-scripts",
  "allow-storage-access-by-user-activation",
  "allow-top-navigation",
  "allow-top-navigation-by-user-activation",
};

impl Rule for IframeMissingSandbox {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        // Also handle React.createElement
        // if is_create_element_call(call_expr) {
        let AstKind::JSXOpeningElement(jsx_el) = node.kind() else {
            return;
        };

        let JSXElementName::Identifier(iden) = &jsx_el.name else {
            return;
        };

        let Some(name) = get_element_type(ctx, jsx_el) else {
            return;
        };

        if name != "iframe" {
            return;
        }

        let Some(sandbox_prop) = has_jsx_prop_lowercase(jsx_el, "sandbox") else {
            ctx.diagnostic(iframe_missing_sandbox_diagnostic(iden.span));
            return;
        };

        // https://github.com/jsx-eslint/eslint-plugin-react/blob/master/lib/rules/iframe-missing-sandbox.js#L17

        dbg!(sandbox_prop);
        match get_prop_value(sandbox_prop) {
            Some(JSXAttributeValue::StringLiteral(str)) => {
                if !str.value.as_str().is_empty() {
                    return;
                }
            }
            Some(JSXAttributeValue::ExpressionContainer(container)) => {
                match &container.expression {
                    JSXExpression::StringLiteral(str) => {
                        if !str.value.is_empty() {
                            return;
                        }
                    }
                    JSXExpression::TemplateLiteral(tmpl) => {
                        if !tmpl.quasis.is_empty()
                            & !tmpl.expressions.is_empty()
                            & tmpl.quasis.iter().any(|q| !q.value.raw.as_str().is_empty())
                        {
                            return;
                        }
                    }
                    expr @ JSXExpression::Identifier(_) => {
                        if !expr.is_undefined() {
                            return;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        r#"<div sandbox="__unknown__" />;"#,
        r#"<iframe sandbox="" />;"#,
        r#"<iframe sandbox={""} />"#,
        r#"React.createElement("iframe", { sandbox: "" });"#,
        r#"<iframe src="foo.htm" sandbox></iframe>"#,
        r#"React.createElement("iframe", { src: "foo.htm", sandbox: true })"#,
        r#"<iframe src="foo.htm" sandbox sandbox></iframe>"#,
        r#"<iframe sandbox="allow-forms"></iframe>"#,
        r#"<iframe sandbox="allow-modals"></iframe>"#,
        r#"<iframe sandbox="allow-orientation-lock"></iframe>"#,
        r#"<iframe sandbox="allow-pointer-lock"></iframe>"#,
        r#"<iframe sandbox="allow-popups"></iframe>"#,
        r#"<iframe sandbox="allow-popups-to-escape-sandbox"></iframe>"#,
        r#"<iframe sandbox="allow-presentation"></iframe>"#,
        r#"<iframe sandbox="allow-same-origin"></iframe>"#,
        r#"<iframe sandbox="allow-scripts"></iframe>"#,
        r#"<iframe sandbox="allow-top-navigation"></iframe>"#,
        r#"<iframe sandbox="allow-top-navigation-by-user-activation"></iframe>"#,
        r#"<iframe sandbox="allow-forms allow-modals"></iframe>"#,
        r#"<iframe sandbox="allow-popups allow-popups-to-escape-sandbox allow-pointer-lock allow-same-origin allow-top-navigation"></iframe>"#,
        r#"React.createElement("iframe", { sandbox: "allow-forms" })"#,
        r#"React.createElement("iframe", { sandbox: "allow-modals" })"#,
        r#"React.createElement("iframe", { sandbox: "allow-orientation-lock" })"#,
        r#"React.createElement("iframe", { sandbox: "allow-pointer-lock" })"#,
        r#"React.createElement("iframe", { sandbox: "allow-popups" })"#,
        r#"React.createElement("iframe", { sandbox: "allow-popups-to-escape-sandbox" })"#,
        r#"React.createElement("iframe", { sandbox: "allow-presentation" })"#,
        r#"React.createElement("iframe", { sandbox: "allow-same-origin" })"#,
        r#"React.createElement("iframe", { sandbox: "allow-scripts" })"#,
        r#"React.createElement("iframe", { sandbox: "allow-top-navigation" })"#,
        r#"React.createElement("iframe", { sandbox: "allow-top-navigation-by-user-activation" })"#,
        r#"React.createElement("iframe", { sandbox: "allow-forms allow-modals" })"#,
        r#"React.createElement("iframe", { sandbox: "allow-popups allow-popups-to-escape-sandbox allow-pointer-lock allow-same-origin allow-top-navigation" })"#,
    ];

    let fail = vec![
        "<iframe></iframe>;",
        "<iframe/>;",
        r#"React.createElement("iframe");"#,
        r#"React.createElement("iframe", {});"#,
        r#"React.createElement("iframe", null);"#,
        r#"<iframe sandbox="__unknown__"></iframe>"#,
        r#"React.createElement("iframe", { sandbox: "__unknown__" })"#,
        r#"<iframe sandbox="allow-popups __unknown__"/>"#,
        r#"<iframe sandbox="__unknown__ allow-popups"/>"#,
        r#"<iframe sandbox=" allow-forms __unknown__ allow-popups __unknown__  "/>"#,
        r#"<iframe sandbox="allow-scripts allow-same-origin"></iframe>;"#,
        r#"<iframe sandbox="allow-same-origin allow-scripts"/>;"#,
    ];

    Tester::new(IframeMissingSandbox::NAME, pass, fail).test_and_snapshot();
}
