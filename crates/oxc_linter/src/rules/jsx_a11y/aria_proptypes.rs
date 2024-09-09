use oxc_ast::{
    ast::{Expression, JSXAttributeItem, JSXAttributeValue},
    AstKind,
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;
use phf::{phf_map, phf_set, Map, Set};

use crate::{
    context::LintContext,
    fixer::{RuleFix, RuleFixer},
    globals::VALID_ARIA_PROPS,
    rule::Rule,
    utils::get_jsx_attribute_name,
    AstNode,
};

#[derive(Clone)]
pub enum AriaPropType {
    String,
    Boolean,
    Id,
    Tristate, // true/false or "mixed"
    Integer,
    Number,
    IdList,
    Token,
    TokenList,
}

struct AriaPropTypeStruct {
    prop_type: AriaPropType,
    allowed_values: Option<Set<&'static str>>,
    allow_undefined: bool,
    allow_boolean_values: bool,
}

// Type
// allowUndefined
// permittedValues
// https://github.com/A11yance/aria-query/blob/main/src/ariaPropsMap.js
// https://github.com/oxc-project/oxc/pull/1810/files
const ARIA_PROP_TYPES: Map<&'static str, AriaPropTypeStruct> = phf_map! {
    "aria-activedescendant" => AriaPropTypeStruct { prop_type: AriaPropType::Id, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-atomic" => AriaPropTypeStruct { prop_type: AriaPropType::Boolean, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-autocomplete" => AriaPropTypeStruct { prop_type: AriaPropType::Token, allowed_values: Some(phf_set! { "inline", "list", "both", "none"}), allow_undefined: false, allow_boolean_values: false },
    "aria-braillelabel" => AriaPropTypeStruct { prop_type: AriaPropType::String, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-brailleroledescription" => AriaPropTypeStruct { prop_type: AriaPropType::String, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-busy" => AriaPropTypeStruct { prop_type: AriaPropType::Boolean, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-checked" => AriaPropTypeStruct { prop_type: AriaPropType::Tristate, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-colcount" => AriaPropTypeStruct { prop_type: AriaPropType::Integer, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-colindex" => AriaPropTypeStruct { prop_type: AriaPropType::Integer, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-colspan" => AriaPropTypeStruct { prop_type: AriaPropType::Integer, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-controls" => AriaPropTypeStruct { prop_type: AriaPropType::IdList, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-current" => AriaPropTypeStruct { prop_type: AriaPropType::Token, allowed_values: Some(phf_set! { "page", "step", "location", "date", "time"}), allow_undefined: false, allow_boolean_values: true },
    "aria-describedby" => AriaPropTypeStruct { prop_type: AriaPropType::IdList, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-description" => AriaPropTypeStruct { prop_type: AriaPropType::String, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-details" => AriaPropTypeStruct { prop_type: AriaPropType::Id, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-disabled" => AriaPropTypeStruct { prop_type: AriaPropType::Boolean, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-dropeffect" => AriaPropTypeStruct { prop_type: AriaPropType::TokenList, allowed_values: Some(phf_set! { "copy", "execute", "link", "move", "none", "popup"}), allow_undefined: false, allow_boolean_values: false },
    "aria-errormessage" => AriaPropTypeStruct { prop_type: AriaPropType::Id, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-expanded" => AriaPropTypeStruct { prop_type: AriaPropType::Boolean, allowed_values: None, allow_undefined: true, allow_boolean_values: false },
    "aria-flowto" => AriaPropTypeStruct { prop_type: AriaPropType::IdList, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-grabbed" => AriaPropTypeStruct { prop_type: AriaPropType::Boolean, allowed_values: None, allow_undefined: true, allow_boolean_values: false },
    "aria-haspopup" => AriaPropTypeStruct { prop_type: AriaPropType::Token, allowed_values: Some(phf_set! { "menu", "listbox", "tree", "grid", "dialog"}), allow_undefined: false, allow_boolean_values: true },
    "aria-hidden" => AriaPropTypeStruct { prop_type: AriaPropType::Boolean, allowed_values: None, allow_undefined: true, allow_boolean_values: false },
    "aria-invalid" => AriaPropTypeStruct { prop_type: AriaPropType::Token, allowed_values: Some(phf_set! { "grammar", "spelling"}), allow_undefined: false, allow_boolean_values: true },
    "aria-keyshortcuts" => AriaPropTypeStruct { prop_type: AriaPropType::String, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-label" => AriaPropTypeStruct { prop_type: AriaPropType::String, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-labelledby" => AriaPropTypeStruct { prop_type: AriaPropType::IdList, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-level" => AriaPropTypeStruct { prop_type: AriaPropType::Integer, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-live" => AriaPropTypeStruct { prop_type: AriaPropType::Token, allowed_values: Some(phf_set! { "assertive", "off", "polite"}), allow_undefined: false, allow_boolean_values: false },
    "aria-modal" => AriaPropTypeStruct { prop_type: AriaPropType::Boolean, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-multiline" => AriaPropTypeStruct { prop_type: AriaPropType::Boolean, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-multiselectable" => AriaPropTypeStruct { prop_type: AriaPropType::Boolean, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-orientation" => AriaPropTypeStruct { prop_type: AriaPropType::Token, allowed_values: Some(phf_set! { "vertical", "undefined", "horizontal"}), allow_undefined: false, allow_boolean_values: false },
    "aria-owns" => AriaPropTypeStruct { prop_type: AriaPropType::IdList, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-placeholder" => AriaPropTypeStruct { prop_type: AriaPropType::String, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-posinset" => AriaPropTypeStruct { prop_type: AriaPropType::Integer, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-pressed" => AriaPropTypeStruct { prop_type: AriaPropType::Tristate, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-readonly" => AriaPropTypeStruct { prop_type: AriaPropType::Boolean, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-relevant" => AriaPropTypeStruct { prop_type: AriaPropType::TokenList, allowed_values: Some(phf_set! { "additions", "all", "removals", "text"}), allow_undefined: false, allow_boolean_values: false },
    "aria-required" => AriaPropTypeStruct { prop_type: AriaPropType::Boolean, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-roledescription" => AriaPropTypeStruct { prop_type: AriaPropType::String, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-rowcount" => AriaPropTypeStruct { prop_type: AriaPropType::Integer, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-rowindex" => AriaPropTypeStruct { prop_type: AriaPropType::Integer, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-rowspan" => AriaPropTypeStruct { prop_type: AriaPropType::Integer, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-selected" => AriaPropTypeStruct { prop_type: AriaPropType::Boolean, allowed_values: None, allow_undefined: true, allow_boolean_values: false },
    "aria-setsize" => AriaPropTypeStruct { prop_type: AriaPropType::Integer, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-sort" => AriaPropTypeStruct { prop_type: AriaPropType::Token, allowed_values: Some(phf_set! { "ascending", "descending", "none", "other"}), allow_undefined: false, allow_boolean_values: false },
    "aria-valuemax" => AriaPropTypeStruct { prop_type: AriaPropType::Number, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-valuemin" => AriaPropTypeStruct { prop_type: AriaPropType::Number, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-valuenow" => AriaPropTypeStruct { prop_type: AriaPropType::Number, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
    "aria-valuetext" => AriaPropTypeStruct { prop_type: AriaPropType::String, allowed_values: None, allow_undefined: false, allow_boolean_values: false },
};

#[derive(Debug, Default, Clone)]
pub struct AriaProptypes;

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
    AriaProptypes,
    correctness, // TODO: change category to `correctness`, `suspicious`, `pedantic`, `perf`, `restriction`, or `style`
             // See <https://oxc.rs/docs/contribute/linter.html#rule-category> for details

);

impl Rule for AriaProptypes {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::JSXAttributeItem(JSXAttributeItem::Attribute(attr)) = node.kind() else {
            return;
        };

        let name = get_jsx_attribute_name(&attr.name).to_lowercase();
        if !name.starts_with("aria-") || !VALID_ARIA_PROPS.contains(&name) {
            return;
        }

        let Some(value) = &attr.value else {
            return;
        };

        let Some(aria_prop_type) = ARIA_PROP_TYPES.get(&name) else {
            return;
        };

        match aria_prop_type.prop_type {
            AriaPropType::Boolean => if validate_boolean(value) {},
            _ => {}
        };

        // dbg!(&ARIA_PROP_TYPES.get(&name).unwrap().allow_undefined);
        dbg!(aria_prop_type.allow_undefined);
    }
}

fn validate_boolean(value: &JSXAttributeValue) -> bool {
    match value {
        JSXAttributeValue::StringLiteral(s) => s.value == "true" || s.value == "false",
        JSXAttributeValue::ExpressionContainer(container) => {
            match container.expression.as_expression() {
                Some(Expression::BooleanLiteral(_)) => true,
                Some(Expression::StringLiteral(s)) => s.value == "true" || s.value == "false",
                _ => false,
            }
        }
        _ => false,
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        r#"<div aria-foo="true" />"#,
        r#"<div abcaria-foo="true" />"#,
        "<div aria-hidden={true} />",
        r#"<div aria-hidden="true" />"#,
        r#"<div aria-hidden={"false"} />"#,
        "<div aria-hidden={!false} />",
        "<div aria-hidden />",
        "<div aria-hidden={false} />",
        "<div aria-hidden={!true} />",
        r#"<div aria-hidden={!"yes"} />"#,
        "<div aria-hidden={foo} />",
        "<div aria-hidden={foo.bar} />",
        "<div aria-hidden={null} />",
        "<div aria-hidden={undefined} />",
        "<div aria-hidden={<div />} />",
        r#"<div aria-label="Close" />"#,
        "<div aria-label={`Close`} />",
        "<div aria-label={foo} />",
        "<div aria-label={foo.bar} />",
        "<div aria-label={null} />",
        "<div aria-label={undefined} />",
        r#"<input aria-invalid={error ? "true" : "false"} />"#,
        r#"<input aria-invalid={undefined ? "true" : "false"} />"#,
        "<div aria-checked={true} />",
        r#"<div aria-checked="true" />"#,
        r#"<div aria-checked={"false"} />"#,
        "<div aria-checked={!false} />",
        "<div aria-checked />",
        "<div aria-checked={false} />",
        "<div aria-checked={!true} />",
        r#"<div aria-checked={!"yes"} />"#,
        "<div aria-checked={foo} />",
        "<div aria-checked={foo.bar} />",
        r#"<div aria-checked="mixed" />"#,
        "<div aria-checked={`mixed`} />",
        "<div aria-checked={null} />",
        "<div aria-checked={undefined} />",
        "<div aria-level={123} />",
        "<div aria-level={-123} />",
        "<div aria-level={+123} />",
        "<div aria-level={~123} />",
        r#"<div aria-level={"123"} />"#,
        "<div aria-level={`123`} />",
        r#"<div aria-level="123" />"#,
        "<div aria-level={foo} />",
        "<div aria-level={foo.bar} />",
        "<div aria-level={null} />",
        "<div aria-level={undefined} />",
        "<div aria-valuemax={123} />",
        "<div aria-valuemax={-123} />",
        "<div aria-valuemax={+123} />",
        "<div aria-valuemax={~123} />",
        r#"<div aria-valuemax={"123"} />"#,
        "<div aria-valuemax={`123`} />",
        r#"<div aria-valuemax="123" />"#,
        "<div aria-valuemax={foo} />",
        "<div aria-valuemax={foo.bar} />",
        "<div aria-valuemax={null} />",
        "<div aria-valuemax={undefined} />",
        r#"<div aria-sort="ascending" />"#,
        r#"<div aria-sort="ASCENDING" />"#,
        r#"<div aria-sort={"ascending"} />"#,
        "<div aria-sort={`ascending`} />",
        r#"<div aria-sort="descending" />"#,
        r#"<div aria-sort={"descending"} />"#,
        "<div aria-sort={`descending`} />",
        r#"<div aria-sort="none" />"#,
        r#"<div aria-sort={"none"} />"#,
        "<div aria-sort={`none`} />",
        r#"<div aria-sort="other" />"#,
        r#"<div aria-sort={"other"} />"#,
        "<div aria-sort={`other`} />",
        "<div aria-sort={foo} />",
        "<div aria-sort={foo.bar} />",
        "<div aria-invalid={true} />",
        r#"<div aria-invalid="true" />"#,
        "<div aria-invalid={false} />",
        r#"<div aria-invalid="false" />"#,
        r#"<div aria-invalid="grammar" />"#,
        r#"<div aria-invalid="spelling" />"#,
        "<div aria-invalid={null} />",
        "<div aria-invalid={undefined} />",
        r#"<div aria-relevant="additions" />"#,
        r#"<div aria-relevant={"additions"} />"#,
        "<div aria-relevant={`additions`} />",
        r#"<div aria-relevant="additions removals" />"#,
        r#"<div aria-relevant="additions additions" />"#,
        r#"<div aria-relevant={"additions removals"} />"#,
        "<div aria-relevant={`additions removals`} />",
        r#"<div aria-relevant="additions removals text" />"#,
        r#"<div aria-relevant={"additions removals text"} />"#,
        "<div aria-relevant={`additions removals text`} />",
        r#"<div aria-relevant="additions removals text all" />"#,
        r#"<div aria-relevant={"additions removals text all"} />"#,
        "<div aria-relevant={`removals additions text all`} />",
        "<div aria-relevant={foo} />",
        "<div aria-relevant={foo.bar} />",
        "<div aria-relevant={null} />",
        "<div aria-relevant={undefined} />",
        r#"<div aria-activedescendant="ascending" />"#,
        r#"<div aria-activedescendant="ASCENDING" />"#,
        r#"<div aria-activedescendant={"ascending"} />"#,
        "<div aria-activedescendant={`ascending`} />",
        r#"<div aria-activedescendant="descending" />"#,
        r#"<div aria-activedescendant={"descending"} />"#,
        "<div aria-activedescendant={`descending`} />",
        r#"<div aria-activedescendant="none" />"#,
        r#"<div aria-activedescendant={"none"} />"#,
        "<div aria-activedescendant={`none`} />",
        r#"<div aria-activedescendant="other" />"#,
        r#"<div aria-activedescendant={"other"} />"#,
        "<div aria-activedescendant={`other`} />",
        "<div aria-activedescendant={foo} />",
        "<div aria-activedescendant={foo.bar} />",
        "<div aria-activedescendant={null} />",
        "<div aria-activedescendant={undefined} />",
        r#"<div aria-labelledby="additions" />"#,
        r#"<div aria-labelledby={"additions"} />"#,
        "<div aria-labelledby={`additions`} />",
        r#"<div aria-labelledby="additions removals" />"#,
        r#"<div aria-labelledby="additions additions" />"#,
        r#"<div aria-labelledby={"additions removals"} />"#,
        "<div aria-labelledby={`additions removals`} />",
        r#"<div aria-labelledby="additions removals text" />"#,
        r#"<div aria-labelledby={"additions removals text"} />"#,
        "<div aria-labelledby={`additions removals text`} />",
        r#"<div aria-labelledby="additions removals text all" />"#,
        r#"<div aria-labelledby={"additions removals text all"} />"#,
        "<div aria-labelledby={`removals additions text all`} />",
        "<div aria-labelledby={foo} />",
        "<div aria-labelledby={foo.bar} />",
        "<div aria-labelledby={null} />",
        "<div aria-labelledby={undefined} />",
    ];

    let fail = vec![
        r#"<div aria-hidden="yes" />"#,
        r#"<div aria-hidden="no" />"#,
        "<div aria-hidden={1234} />",
        "<div aria-hidden={`${abc}`} />",
        "<div aria-label />",
        "<div aria-label={true} />",
        "<div aria-label={false} />",
        "<div aria-label={1234} />",
        "<div aria-label={!true} />",
        r#"<div aria-checked="yes" />"#,
        r#"<div aria-checked="no" />"#,
        "<div aria-checked={1234} />",
        "<div aria-checked={`${abc}`} />",
        r#"<div aria-level="yes" />"#,
        r#"<div aria-level="no" />"#,
        "<div aria-level={`abc`} />",
        "<div aria-level={true} />",
        "<div aria-level />",
        r#"<div aria-level={"false"} />"#,
        r#"<div aria-level={!"false"} />"#,
        r#"<div aria-valuemax="yes" />"#,
        r#"<div aria-valuemax="no" />"#,
        "<div aria-valuemax={`abc`} />",
        "<div aria-valuemax={true} />",
        "<div aria-valuemax />",
        r#"<div aria-valuemax={"false"} />"#,
        r#"<div aria-valuemax={!"false"} />"#,
        r#"<div aria-sort="" />"#,
        r#"<div aria-sort="descnding" />"#,
        "<div aria-sort />",
        "<div aria-sort={true} />",
        r#"<div aria-sort={"false"} />"#,
        r#"<div aria-sort="ascending descending" />"#,
        r#"<div aria-relevant="" />"#,
        r#"<div aria-relevant="foobar" />"#,
        "<div aria-relevant />",
        "<div aria-relevant={true} />",
        r#"<div aria-relevant={"false"} />"#,
        r#"<div aria-relevant="additions removalss" />"#,
        r#"<div aria-relevant="additions removalss " />"#,
    ];

    Tester::new(AriaProptypes::NAME, pass, fail).test_and_snapshot();
}
