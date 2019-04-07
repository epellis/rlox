use crate::statement::Stmt;
use crate::expression::Expr;

fn visit_block(stmt: Stmt::Block) {
    if let Stmt::Block(statements) = stmt {
        begin_scope();



        end_scope();
    }
}
