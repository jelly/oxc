use oxc_ast::{ast::JSXAttributeItem, AstKind};
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
    Tristate,
    Integer,
    IdList,
    Token,
}

enum AriaPropTypeValue {
    String(String),
    Bool(bool),
}

struct AriaPropTypeStruct {
    prop_type: AriaPropType,
    allowed_values: Option<Set<&'static str>>,
    allow_undefined: bool,
}

// Type
// allowUndefined
// permittedValues
// https://github.com/A11yance/aria-query/blob/main/src/ariaPropsMap.js
// https://github.com/oxc-project/oxc/pull/1810/files
const ARIA_PROP_TYPES: Map<&'static str, AriaPropTypeStruct> = phf_map! {
    "aria-activedescendant" => AriaPropTypeStruct { prop_type: AriaPropType::Id, allowed_values: None, allow_undefined: false },
    "aria-atomic" => AriaPropTypeStruct { prop_type: AriaPropType::Boolean, allowed_values: None, allow_undefined: false },
    "aria-autocomplete" => AriaPropTypeStruct { prop_type: AriaPropType::Token, allowed_values: Some(phf_set! { "inline"} ), allow_undefined: false },
    "aria-braillelabel" => AriaPropTypeStruct { prop_type: AriaPropType::String, allowed_values: None, allow_undefined: false },
    // "aria-braillelabel" => AriaPropType::String,
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

        if let Some(aria_prop_type) = ARIA_PROP_TYPES.get(&name) {
            // dbg!(&ARIA_PROP_TYPES.get(&name).unwrap().allow_undefined);
            dbg!(aria_prop_type.allow_undefined);
        }
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
