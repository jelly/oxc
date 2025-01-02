use oxc_allocator::Vec;
use oxc_ast::ast::*;
use oxc_traverse::{traverse_mut_with_ctx, ReusableTraverseCtx, Traverse, TraverseCtx};

mod collapse_variable_declarations;
mod convert_to_dotted_properties;
mod exploit_assigns;
mod minimize_exit_points;
mod normalize;
mod peephole_fold_constants;
mod peephole_minimize_conditions;
mod peephole_remove_dead_code;
mod peephole_replace_known_methods;
mod peephole_substitute_alternate_syntax;
mod remove_syntax;
mod remove_unused_code;
mod statement_fusion;

pub use collapse_variable_declarations::CollapseVariableDeclarations;
pub use convert_to_dotted_properties::ConvertToDottedProperties;
pub use exploit_assigns::ExploitAssigns;
pub use minimize_exit_points::MinimizeExitPoints;
pub use normalize::Normalize;
pub use peephole_fold_constants::PeepholeFoldConstants;
pub use peephole_minimize_conditions::PeepholeMinimizeConditions;
pub use peephole_remove_dead_code::PeepholeRemoveDeadCode;
pub use peephole_replace_known_methods::PeepholeReplaceKnownMethods;
pub use peephole_substitute_alternate_syntax::PeepholeSubstituteAlternateSyntax;
pub use remove_syntax::RemoveSyntax;
pub use remove_unused_code::RemoveUnusedCode;
pub use statement_fusion::StatementFusion;

use crate::CompressOptions;

pub trait CompressorPass<'a>: Traverse<'a> {
    fn build(&mut self, program: &mut Program<'a>, ctx: &mut ReusableTraverseCtx<'a>);
}

pub struct PeepholeOptimizations {
    x0_statement_fusion: StatementFusion,
    x1_minimize_exit_points: MinimizeExitPoints,
    x2_exploit_assigns: ExploitAssigns,
    x3_collapse_variable_declarations: CollapseVariableDeclarations,
    x4_peephole_remove_dead_code: PeepholeRemoveDeadCode,
    x5_peephole_minimize_conditions: PeepholeMinimizeConditions,
    x6_peephole_substitute_alternate_syntax: PeepholeSubstituteAlternateSyntax,
    x7_peephole_replace_known_methods: PeepholeReplaceKnownMethods,
    x8_peephole_fold_constants: PeepholeFoldConstants,
    x9_convert_to_dotted_properties: ConvertToDottedProperties,
}

impl PeepholeOptimizations {
    pub fn new(in_fixed_loop: bool, options: CompressOptions) -> Self {
        Self {
            x0_statement_fusion: StatementFusion::new(),
            x1_minimize_exit_points: MinimizeExitPoints::new(),
            x2_exploit_assigns: ExploitAssigns::new(),
            x3_collapse_variable_declarations: CollapseVariableDeclarations::new(),
            x4_peephole_remove_dead_code: PeepholeRemoveDeadCode::new(),
            x5_peephole_minimize_conditions: PeepholeMinimizeConditions::new(in_fixed_loop),
            x6_peephole_substitute_alternate_syntax: PeepholeSubstituteAlternateSyntax::new(
                options,
                in_fixed_loop,
            ),
            x7_peephole_replace_known_methods: PeepholeReplaceKnownMethods::new(),
            x8_peephole_fold_constants: PeepholeFoldConstants::new(),
            x9_convert_to_dotted_properties: ConvertToDottedProperties::new(in_fixed_loop),
        }
    }

    fn reset_changed(&mut self) {
        self.x0_statement_fusion.changed = false;
        self.x1_minimize_exit_points.changed = false;
        self.x2_exploit_assigns.changed = false;
        self.x3_collapse_variable_declarations.changed = false;
        self.x4_peephole_remove_dead_code.changed = false;
        self.x5_peephole_minimize_conditions.changed = false;
        self.x6_peephole_substitute_alternate_syntax.changed = false;
        self.x7_peephole_replace_known_methods.changed = false;
        self.x8_peephole_fold_constants.changed = false;
    }

    fn changed(&self) -> bool {
        self.x0_statement_fusion.changed
            || self.x1_minimize_exit_points.changed
            || self.x2_exploit_assigns.changed
            || self.x3_collapse_variable_declarations.changed
            || self.x4_peephole_remove_dead_code.changed
            || self.x5_peephole_minimize_conditions.changed
            || self.x6_peephole_substitute_alternate_syntax.changed
            || self.x7_peephole_replace_known_methods.changed
            || self.x8_peephole_fold_constants.changed
    }

    pub fn run_in_loop<'a>(
        &mut self,
        program: &mut Program<'a>,
        ctx: &mut ReusableTraverseCtx<'a>,
    ) {
        let mut i = 0;
        loop {
            self.reset_changed();
            self.build(program, ctx);
            if !self.changed() {
                break;
            }
            if i > 10 {
                debug_assert!(false, "Ran loop more than 10 times.");
                break;
            }
            i += 1;
        }
    }
}

impl<'a> CompressorPass<'a> for PeepholeOptimizations {
    fn build(&mut self, program: &mut Program<'a>, ctx: &mut ReusableTraverseCtx<'a>) {
        traverse_mut_with_ctx(self, program, ctx);
    }
}

impl<'a> Traverse<'a> for PeepholeOptimizations {
    fn exit_program(&mut self, program: &mut Program<'a>, ctx: &mut TraverseCtx<'a>) {
        self.x0_statement_fusion.exit_program(program, ctx);
        self.x4_peephole_remove_dead_code.exit_program(program, ctx);
    }

    fn exit_function_body(&mut self, body: &mut FunctionBody<'a>, ctx: &mut TraverseCtx<'a>) {
        self.x0_statement_fusion.exit_function_body(body, ctx);
    }

    fn exit_statements(&mut self, stmts: &mut Vec<'a, Statement<'a>>, ctx: &mut TraverseCtx<'a>) {
        self.x1_minimize_exit_points.exit_statements(stmts, ctx);
        self.x2_exploit_assigns.exit_statements(stmts, ctx);
        self.x3_collapse_variable_declarations.exit_statements(stmts, ctx);
        self.x4_peephole_remove_dead_code.exit_statements(stmts, ctx);
        self.x5_peephole_minimize_conditions.exit_statements(stmts, ctx);
    }

    fn exit_statement(&mut self, stmt: &mut Statement<'a>, ctx: &mut TraverseCtx<'a>) {
        self.x4_peephole_remove_dead_code.exit_statement(stmt, ctx);
        self.x5_peephole_minimize_conditions.exit_statement(stmt, ctx);
    }

    fn exit_block_statement(&mut self, block: &mut BlockStatement<'a>, ctx: &mut TraverseCtx<'a>) {
        self.x0_statement_fusion.exit_block_statement(block, ctx);
    }

    fn exit_return_statement(&mut self, stmt: &mut ReturnStatement<'a>, ctx: &mut TraverseCtx<'a>) {
        self.x6_peephole_substitute_alternate_syntax.exit_return_statement(stmt, ctx);
    }

    fn exit_variable_declaration(
        &mut self,
        decl: &mut VariableDeclaration<'a>,
        ctx: &mut TraverseCtx<'a>,
    ) {
        self.x6_peephole_substitute_alternate_syntax.exit_variable_declaration(decl, ctx);
    }

    fn exit_expression(&mut self, expr: &mut Expression<'a>, ctx: &mut TraverseCtx<'a>) {
        self.x4_peephole_remove_dead_code.exit_expression(expr, ctx);
        self.x5_peephole_minimize_conditions.exit_expression(expr, ctx);
        self.x6_peephole_substitute_alternate_syntax.exit_expression(expr, ctx);
        self.x7_peephole_replace_known_methods.exit_expression(expr, ctx);
        self.x8_peephole_fold_constants.exit_expression(expr, ctx);
    }

    fn enter_call_expression(&mut self, expr: &mut CallExpression<'a>, ctx: &mut TraverseCtx<'a>) {
        self.x6_peephole_substitute_alternate_syntax.enter_call_expression(expr, ctx);
    }

    fn exit_call_expression(&mut self, expr: &mut CallExpression<'a>, ctx: &mut TraverseCtx<'a>) {
        self.x6_peephole_substitute_alternate_syntax.exit_call_expression(expr, ctx);
    }

    fn exit_property_key(&mut self, key: &mut PropertyKey<'a>, ctx: &mut TraverseCtx<'a>) {
        self.x9_convert_to_dotted_properties.exit_property_key(key, ctx);
    }

    fn exit_member_expression(
        &mut self,
        expr: &mut MemberExpression<'a>,
        ctx: &mut TraverseCtx<'a>,
    ) {
        self.x9_convert_to_dotted_properties.exit_member_expression(expr, ctx);
    }

    fn exit_catch_clause(&mut self, catch: &mut CatchClause<'a>, ctx: &mut TraverseCtx<'a>) {
        self.x6_peephole_substitute_alternate_syntax.exit_catch_clause(catch, ctx);
    }
}

pub struct DeadCodeElimination {
    x1_peephole_fold_constants: PeepholeFoldConstants,
    x2_peephole_remove_dead_code: PeepholeRemoveDeadCode,
}

impl DeadCodeElimination {
    pub fn new() -> Self {
        Self {
            x1_peephole_fold_constants: PeepholeFoldConstants::new(),
            x2_peephole_remove_dead_code: PeepholeRemoveDeadCode::new(),
        }
    }
}

impl<'a> CompressorPass<'a> for DeadCodeElimination {
    fn build(&mut self, program: &mut Program<'a>, ctx: &mut ReusableTraverseCtx<'a>) {
        traverse_mut_with_ctx(self, program, ctx);
    }
}

impl<'a> Traverse<'a> for DeadCodeElimination {
    fn exit_statement(&mut self, stmt: &mut Statement<'a>, ctx: &mut TraverseCtx<'a>) {
        self.x2_peephole_remove_dead_code.exit_statement(stmt, ctx);
    }

    fn exit_program(&mut self, program: &mut Program<'a>, ctx: &mut TraverseCtx<'a>) {
        self.x2_peephole_remove_dead_code.exit_program(program, ctx);
    }

    fn exit_statements(&mut self, stmts: &mut Vec<'a, Statement<'a>>, ctx: &mut TraverseCtx<'a>) {
        self.x2_peephole_remove_dead_code.exit_statements(stmts, ctx);
    }

    fn exit_expression(&mut self, expr: &mut Expression<'a>, ctx: &mut TraverseCtx<'a>) {
        self.x1_peephole_fold_constants.exit_expression(expr, ctx);
        self.x2_peephole_remove_dead_code.exit_expression(expr, ctx);
    }
}
