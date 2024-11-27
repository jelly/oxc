use crate::ToJsString;
use oxc_ast::ast::*;

pub trait ArrayJoin<'a> {
    /// `Array.prototype.join ( separator )`
    /// <https://tc39.es/ecma262/#sec-array.prototype.join>
    fn array_join(&self, separator: Option<&str>) -> Option<String>;
}

impl<'a> ArrayJoin<'a> for ArrayExpression<'a> {
    fn array_join(&self, separator: Option<&str>) -> Option<String> {
        let strings =
            self.elements.iter().map(ToJsString::to_js_string).collect::<Option<Vec<_>>>();
        strings
            .map(|v| v.iter().map(AsRef::as_ref).collect::<Vec<_>>().join(separator.unwrap_or(",")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::{Allocator, CloneIn};
    use oxc_ast::AstBuilder;
    use oxc_span::SPAN;

    #[test]
    fn test() {
        let allocator = Allocator::default();
        let ast = AstBuilder::new(&allocator);
        let mut elements = ast.vec();
        elements.push(ast.array_expression_element_elision(SPAN));
        elements.push(ArrayExpressionElement::NullLiteral(ast.alloc(ast.null_literal(SPAN))));
        elements.push(ArrayExpressionElement::NumericLiteral(ast.alloc(ast.numeric_literal(
            SPAN,
            42f64,
            "42",
            NumberBase::Decimal,
        ))));
        elements.push(ArrayExpressionElement::StringLiteral(
            ast.alloc(ast.string_literal(SPAN, "foo", None)),
        ));
        elements.push(ArrayExpressionElement::BooleanLiteral(
            ast.alloc(ast.boolean_literal(SPAN, true)),
        ));
        elements.push(ArrayExpressionElement::BigIntLiteral(ast.alloc(ast.big_int_literal(
            SPAN,
            "42n",
            BigintBase::Decimal,
        ))));
        let array = ast.array_expression(SPAN, elements.clone_in(&allocator), None);
        let mut array2 = array.clone_in(&allocator);
        array2.elements.push(ArrayExpressionElement::ArrayExpression(ast.alloc(array)));
        array2.elements.push(ArrayExpressionElement::ObjectExpression(
            ast.alloc(ast.object_expression(SPAN, ast.vec(), None)),
        ));
        let joined = array2.array_join(Some("_"));
        assert_eq!(joined, Some("__42_foo_true_42n_,,42,foo,true,42n_[object Object]".to_string()));

        let joined2 = array2.array_join(None);
        // By default, in `Array.prototype.toString`, the separator is a comma. However, in `Array.prototype.join`, the separator is none if not given.
        assert_eq!(
            joined2,
            Some(",,42,foo,true,42n,,,42,foo,true,42n,[object Object]".to_string())
        );
    }
}