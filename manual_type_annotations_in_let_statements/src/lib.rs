#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_span;

use rustc_errors::Applicability;
use rustc_hir::ExprKind;
use rustc_lint::{LateLintPass, LintContext};
use rustc_span::Span;

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

/// Primitive types that support literal suffixes.
const SUFFIX_TYPES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize",
    "u8", "u16", "u32", "u64", "u128", "usize",
    "f32", "f64",
];

/// Try to suggest adding a type suffix to a numeric literal.
fn try_suggest_literal_suffix(
    remove_span: Span,
    lit: &rustc_hir::Lit,
    type_str: &str,
) -> Option<Vec<(Span, String)>> {
    match lit.node {
        rustc_ast::LitKind::Int(..) | rustc_ast::LitKind::Float(..) => {
            if SUFFIX_TYPES.contains(&type_str) {
                Some(vec![
                    (remove_span, String::new()),
                    (lit.span.shrink_to_hi(), type_str.to_owned()),
                ])
            } else {
                // Non-primitive type, just remove annotation
                Some(vec![(remove_span, String::new())])
            }
        }
        _ => {
            // Other literals (strings, chars, etc.) - just remove annotation
            Some(vec![(remove_span, String::new())])
        }
    }
}

/// Extracts generic arguments span from a type HIR node.
/// For `Vec<usize>` returns the span of `<usize>`.
/// For `HashMap<String, i32>` returns the span of `<String, i32>`.
fn get_generic_args_span(ty_hir: &rustc_hir::Ty<'_>) -> Option<Span> {
    if let rustc_hir::TyKind::Path(qpath) = &ty_hir.kind {
        let segment = match qpath {
            rustc_hir::QPath::Resolved(_, path) => path.segments.last()?,
            rustc_hir::QPath::TypeRelative(_, segment) => *segment,
        };
        // Get the args span if there are generic arguments
        if let Some(args) = segment.args {
            return Some(args.span_ext);
        }
    }
    None
}

/// Builds a multipart suggestion for moving the type annotation.
/// Returns None if we can't build a suggestion (e.g., macro-generated code).
fn build_suggestion(
    cx: &rustc_lint::LateContext<'_>,
    stmt: &rustc_hir::LetStmt<'_>,
) -> Option<Vec<(Span, String)>> {
    let ty_hir = stmt.ty?;
    let init = stmt.init?;

    // Skip macro-generated code
    if stmt.span.from_expansion() {
        return None;
    }

    let source_map = cx.sess().source_map();

    // Get the type string from source
    let type_str = source_map.span_to_snippet(ty_hir.span).ok()?;

    // The span to remove: `: Type` (from after pattern to end of type annotation)
    let remove_span = stmt.pat.span.between(ty_hir.span).to(ty_hir.span);

    match &init.kind {
        ExprKind::MethodCall(segment, receiver, _args, _span) => {
            let method_name = segment.ident.name.as_str();

            match method_name {
                // Methods that support turbofish
                "collect" | "from_iter" => {
                    Some(vec![
                        (remove_span, String::new()),
                        (segment.ident.span.shrink_to_hi(), format!("::<{}>", type_str)),
                    ])
                }
                // into() -> T::from(receiver)
                "into" => {
                    let receiver_str = source_map.span_to_snippet(receiver.span).ok()?;
                    Some(vec![
                        (remove_span, String::new()),
                        (init.span, format!("{}::from({})", type_str, receiver_str)),
                    ])
                }
                // try_into() -> T::try_from(receiver)
                "try_into" => {
                    let receiver_str = source_map.span_to_snippet(receiver.span).ok()?;
                    Some(vec![
                        (remove_span, String::new()),
                        (init.span, format!("{}::try_from({})", type_str, receiver_str)),
                    ])
                }
                // Other methods (to_string, clone, etc.) - just remove annotation
                _ => Some(vec![(remove_span, String::new())]),
            }
        }
        ExprKind::Call(callee, _args) => {
            // For Type::new() style calls - add turbofish to the type path
            if let ExprKind::Path(rustc_hir::QPath::TypeRelative(callee_ty, _segment)) = &callee.kind
            {
                // Extract just the generic args from the type annotation
                if let Some(args_span) = get_generic_args_span(ty_hir) {
                    let args_str = source_map.span_to_snippet(args_span).ok()?;
                    // Insert ::<Args> after the type name in the callee
                    Some(vec![
                        (remove_span, String::new()),
                        (callee_ty.span.shrink_to_hi(), format!("::{}", args_str)),
                    ])
                } else {
                    // No generic args - just remove annotation
                    Some(vec![(remove_span, String::new())])
                }
            } else {
                // Other call types - just remove the annotation
                Some(vec![(remove_span, String::new())])
            }
        }
        ExprKind::Lit(lit) => {
            // For numeric literals, add type suffix
            try_suggest_literal_suffix(remove_span, lit, &type_str)
        }
        ExprKind::Unary(rustc_hir::UnOp::Neg | rustc_hir::UnOp::Not, inner) => {
            // For negated literals like `-10`, add suffix to the inner literal
            if let ExprKind::Lit(lit) = &inner.kind {
                try_suggest_literal_suffix(remove_span, lit, &type_str)
            } else {
                Some(vec![(remove_span, String::new())])
            }
        }
        ExprKind::Path(rustc_hir::QPath::Resolved(_, path)) => {
            // Handle None - suggest None::<T> where T is the inner type of Option<T>
            if let Some(segment) = path.segments.last()
                && segment.ident.name.as_str() == "None" {
                    // Extract inner type from Option<T>
                    if let Some(args_span) = get_generic_args_span(ty_hir) {
                        let args_str = source_map.span_to_snippet(args_span).ok()?;
                        return Some(vec![
                            (remove_span, String::new()),
                            (segment.ident.span.shrink_to_hi(), format!("::{}", args_str)),
                        ]);
                    }
                }
            // Other paths - just remove annotation
            Some(vec![(remove_span, String::new())])
        }
        _ => {
            // For other expressions - just remove type annotation
            Some(vec![(remove_span, String::new())])
        }
    }
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
                if let Some(suggestions) = build_suggestion(cx, stmt) {
                    diag.multipart_suggestion(
                        "move type to the right-hand side or remove it",
                        suggestions,
                        Applicability::MaybeIncorrect,
                    );
                } else {
                    diag.help("provide type annotations on the right-hand side of the let, e.g., using turbofish, or remove them altogether if they're superfluous");
                }
            },
        );
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
