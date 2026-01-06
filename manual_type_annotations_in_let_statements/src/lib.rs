#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;

use rustc_lint::{LateLintPass, LintContext};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Warns on manual type annotations in `let` statements.
    ///
    /// ### Why is this bad?
    ///
    /// In many cases, Rust can infer the type of a variable from the right-hand side of a `let`
    /// statement. Adding an explicit type annotation in such cases is redundant and can make the
    /// code less readable. Sometimes, type annotations are necessary, however, in those cases,
    /// prefer to add type annotations to the right-hand side expression instead.
    ///
    /// ### Example
    ///
    /// ```rust
    /// let x: usize = 5;
    /// let y: Vec<i32> = [1, 2, 3].iter().copied().collect();
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// let x = 5usize;
    /// let y = [1, 2, 3].iter().copied().collect::<Vec<i32>>();
    /// ```
    ///
    pub MANUAL_TYPE_ANNOTATION_IN_LET_STATEMENTS,
    Warn,
    "manual type annotation in let statement where it should be inferred"
}

impl<'tcx> LateLintPass<'tcx> for ManualTypeAnnotationInLetStatements {
    fn check_local(
        &mut self,
        cx: &rustc_lint::LateContext<'tcx>,
        stmt: &'tcx rustc_hir::LetStmt<'tcx>,
    ) {
        if stmt.ty.is_none() {
            return;
        }

        cx.span_lint(
            MANUAL_TYPE_ANNOTATION_IN_LET_STATEMENTS,
            stmt.span,
            |diag| {
                diag.help("provide type annotations on the right-hand side of the let, e.g., using turbofish, or remove them altogether if they're superfluous");
            },
        );
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
