#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use std::sync::LazyLock;

use clippy_utils::paths::{PathNS, lookup_path};

use rustc_errors::Applicability;
use rustc_lint::{LateLintPass, LintContext};
use rustc_middle::ty;
use rustc_span::symbol::Symbol;

static ALLOC_STRING_STRING: LazyLock<[Symbol; 3]> = LazyLock::new(|| {
    [
        Symbol::intern("alloc"),
        Symbol::intern("string"),
        Symbol::intern("String"),
    ]
});

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Warns on using `to_string()` on `String` or `&str` to obtain a `String` when `.to_owned()`
    /// should be preferred, as it more clearly expresses the intent of creating an owned `String`.
    ///
    /// ### Why is this bad?
    ///
    /// Using `.to_owned()` is generally more idiomatic when considering the intent of obtaining an
    /// owned version of a type.
    ///
    /// ### Example
    ///
    /// ```rust
    /// let a = "hello".to_string();
    /// let b = "hello";
    /// let c = &&b;
    ///
    /// let d = c.to_string();
    /// let e = d.to_string();
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// let a = "hello".to_owned();
    /// let b = "hello";
    /// let c = &&b;
    ///
    /// let d = c.to_owned();
    /// let e = d.to_owned();
    /// ```
    ///
    pub TO_STRING_ON_STRING_TYPES,
    Warn,
    "using `to_string` on `String` or `&str` to obtain a `String` when `to_owned` should be preferred"
}

fn matches_string_like(ty: &ty::Ty<'_>, cx: &rustc_lint::LateContext<'_>) -> bool {
    let ty = ty.peel_refs();

    if ty.is_str() {
        return true;
    }

    let ty::Adt(t, _) = ty.kind() else {
        return false;
    };

    lookup_path(cx.tcx, PathNS::Type, &*ALLOC_STRING_STRING).contains(&t.did())
}

impl<'tcx> LateLintPass<'tcx> for ToStringOnStringTypes {
    fn check_expr(
        &mut self,
        cx: &rustc_lint::LateContext<'tcx>,
        expr: &'tcx rustc_hir::Expr<'tcx>,
    ) {
        let rustc_hir::ExprKind::MethodCall(path_segment, mexpr, ..) = &expr.kind else {
            return;
        };

        if path_segment.ident.name.as_str() != "to_string" {
            return;
        }

        let Some(ty) = cx.typeck_results().expr_ty_opt(mexpr) else {
            return;
        };

        if !matches_string_like(&ty, cx) {
            return;
        }

        cx.span_lint(TO_STRING_ON_STRING_TYPES, expr.span, |diag| {
            diag.span_suggestion_hidden(
                path_segment.ident.span,
                "use `to_owned` rather than `to_string` to obtain a `String`",
                "to_owned".to_owned(),
                Applicability::MachineApplicable,
            );
        });
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
