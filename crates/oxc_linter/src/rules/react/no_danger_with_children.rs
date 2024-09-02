use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{
    context::LintContext,
    fixer::{RuleFix, RuleFixer},
    rule::Rule,
    AstNode,
};

#[derive(Debug, Default, Clone)]
pub struct NoDangerWithChildren;

declare_oxc_lint!(
    /// ### What it does
    ///
    ///
    /// ### Why is this bad?
    ///
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```jsx
    /// FIXME: Tests will fail if examples are missing or syntactically incorrect.
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```jsx
    /// FIXME: Tests will fail if examples are missing or syntactically incorrect.
    /// ```
    NoDangerWithChildren,
    nursery, // TODO: change category to `correctness`, `suspicious`, `pedantic`, `perf`, `restriction`, or `style`
             // See <https://oxc.rs/docs/contribute/linter.html#rule-category> for details

    pending  // TODO: describe fix capabilities. Remove if no fix can be done,
             // keep at 'pending' if you think one could be added but don't know how.
             // Options are 'fix', 'fix_dangerous', 'suggestion', and 'conditional_fix_suggestion'
);

impl Rule for NoDangerWithChildren {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
            AstKind::JSXAttributeItem(JSXAttributeItem::Attribute(attr)) => {
                let JSXAttributeName::Identifier(attr_ident) = &attr.name else {
                    return;
                };
                if attr_ident.name == "children" {
                    ctx.diagnostic(no_children_prop_diagnostic(attr_ident.span));
                }
            }
            AstKind::CallExpression(call_expr) => {
                if is_create_element_call(call_expr) {
                    if let Some(Argument::ObjectExpression(obj_expr)) = call_expr.arguments.get(1) {
                        if let Some(span) = obj_expr.properties.iter().find_map(|prop| {
                            if let ObjectPropertyKind::ObjectProperty(prop) = prop {
                                if prop.key.is_specific_static_name("children") {
                                    return Some(prop.key.span());
                                }
                            }

                            None
                        }) {
                            ctx.diagnostic(no_children_prop_diagnostic(span));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn should_run(&self, ctx: &LintContext) -> bool {
        ctx.source_type().is_jsx()
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        "<div>Children</div>",
        "<div {...props} />",
        r#"<div dangerouslySetInnerHTML={{ __html: "HTML" }} />"#,
        r#"<div children="Children" />"#,
        r#"
			        const props = { dangerouslySetInnerHTML: { __html: "HTML" } };
			        <div {...props} />
			      "#,
        r#"
			        const moreProps = { className: "eslint" };
			        const props = { children: "Children", ...moreProps };
			        <div {...props} />
			      "#,
        r#"
			        const otherProps = { children: "Children" };
			        const { a, b, ...props } = otherProps;
			        <div {...props} />
			      "#,
        "<Hello>Children</Hello>",
        r#"<Hello dangerouslySetInnerHTML={{ __html: "HTML" }} />"#,
        r#"
			        <Hello dangerouslySetInnerHTML={{ __html: "HTML" }}>
			        </Hello>
			      "#,
        r#"React.createElement("div", { dangerouslySetInnerHTML: { __html: "HTML" } });"#,
        r#"React.createElement("div", {}, "Children");"#,
        r#"React.createElement("Hello", { dangerouslySetInnerHTML: { __html: "HTML" } });"#,
        r#"React.createElement("Hello", {}, "Children");"#,
        "<Hello {...undefined}>Children</Hello>",
        r#"React.createElement("Hello", undefined, "Children")"#,
        "
			        const props = {...props, scratch: {mode: 'edit'}};
			        const component = shallow(<TaskEditableTitle {...props} />);
			      ",
    ];

    let fail = vec![
        r#"
			        <div dangerouslySetInnerHTML={{ __html: "HTML" }}>
			          Children
			        </div>
			      "#,
        r#"<div dangerouslySetInnerHTML={{ __html: "HTML" }} children="Children" />"#,
        r#"
			        const props = { dangerouslySetInnerHTML: { __html: "HTML" } };
			        <div {...props}>Children</div>
			      "#,
        r#"
			        const props = { children: "Children", dangerouslySetInnerHTML: { __html: "HTML" } };
			        <div {...props} />
			      "#,
        r#"
			        <Hello dangerouslySetInnerHTML={{ __html: "HTML" }}>
			          Children
			        </Hello>
			      "#,
        r#"<Hello dangerouslySetInnerHTML={{ __html: "HTML" }} children="Children" />"#,
        r#"<Hello dangerouslySetInnerHTML={{ __html: "HTML" }}> </Hello>"#,
        r#"
			        React.createElement(
			          "div",
			          { dangerouslySetInnerHTML: { __html: "HTML" } },
			          "Children"
			        );
			      "#,
        r#"
			        React.createElement(
			          "div",
			          {
			            dangerouslySetInnerHTML: { __html: "HTML" },
			            children: "Children",
			          }
			        );
			      "#,
        r#"
			        React.createElement(
			          "Hello",
			          { dangerouslySetInnerHTML: { __html: "HTML" } },
			          "Children"
			        );
			      "#,
        r#"
			        React.createElement(
			          "Hello",
			          {
			            dangerouslySetInnerHTML: { __html: "HTML" },
			            children: "Children",
			          }
			        );
			      "#,
        r#"
			        const props = { dangerouslySetInnerHTML: { __html: "HTML" } };
			        React.createElement("div", props, "Children");
			      "#,
        r#"
			        const props = { children: "Children", dangerouslySetInnerHTML: { __html: "HTML" } };
			        React.createElement("div", props);
			      "#,
        r#"
			        const moreProps = { children: "Children" };
			        const otherProps = { ...moreProps };
			        const props = { ...otherProps, dangerouslySetInnerHTML: { __html: "HTML" } };
			        React.createElement("div", props);
			      "#,
    ];

    Tester::new(NoDangerWithChildren::NAME, pass, fail).test_and_snapshot();
}
