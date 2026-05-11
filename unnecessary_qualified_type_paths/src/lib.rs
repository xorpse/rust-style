#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_span;

use rustc_hir::def::{DefKind, Res};
use rustc_hir::{AmbigArg, PolyTraitRef, QPath, Ty, TyKind};
use rustc_lint::{LateLintPass, LintContext};
use rustc_span::Span;
use rustc_span::symbol::kw;

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Warns on unnecessarily qualified type paths in explicit type syntax.
    ///
    /// ### Why is this bad?
    ///
    /// Writing full paths such as `crate::module::Type` or `std::fs::File` in signatures and
    /// other explicit type positions is usually harder to read and maintain than importing the
    /// type directly. In case of name clashes, prefer renaming the import or using a short
    /// disambiguating module such as `io::Result`.
    ///
    /// ### Example
    ///
    /// ```rust
    /// fn open(path: std::path::PathBuf) -> std::io::Result<std::fs::File> {
    ///     todo!()
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// use std::fs::File;
    /// use std::io;
    /// use std::path::PathBuf;
    ///
    /// fn open(path: PathBuf) -> io::Result<File> {
    ///     todo!()
    /// }
    /// ```
    pub UNNECESSARY_QUALIFIED_TYPE_PATHS,
    Warn,
    "using a fully qualified type path where an import or shorter module path would be clearer"
}

fn starts_with_forbidden_prefix(snippet: &str) -> bool {
    matches!(
        snippet.trim_start(),
        s if s.starts_with("crate::")
            || s.starts_with("self::")
            || s.starts_with("super::")
            || s.starts_with("::")
    )
}

fn non_root_segment_count(path: &rustc_hir::Path<'_>) -> usize {
    path.segments
        .iter()
        .filter(|segment| segment.ident.name != kw::PathRoot)
        .count()
}

fn begins_with_external_crate(path: &rustc_hir::Path<'_>) -> bool {
    let Some(first_segment) = path
        .segments
        .iter()
        .find(|segment| segment.ident.name != kw::PathRoot)
    else {
        return false;
    };

    match first_segment.res {
        Res::Def(DefKind::Mod, did) => did.is_crate_root() && !did.is_local(),
        Res::Def(DefKind::ExternCrate, _) => true,
        _ => false,
    }
}

fn should_lint_path(cx: &rustc_lint::LateContext<'_>, path: &rustc_hir::Path<'_>, span: Span) -> bool {
    if span.from_expansion() {
        return false;
    }

    let Ok(snippet) = cx.sess().source_map().span_to_snippet(span) else {
        return false;
    };

    if starts_with_forbidden_prefix(&snippet) {
        return true;
    }

    match non_root_segment_count(path) {
        0 | 1 => false,
        2 => begins_with_external_crate(path),
        _ => true,
    }
}

fn lint_path(cx: &rustc_lint::LateContext<'_>, span: Span) {
    cx.span_lint(UNNECESSARY_QUALIFIED_TYPE_PATHS, span, |diag| {
        diag.help(
            "import the type directly, rename the import if needed, or shorten the path to the last disambiguating module",
        );
    });
}

impl<'tcx> LateLintPass<'tcx> for UnnecessaryQualifiedTypePaths {
    fn check_ty(&mut self, cx: &rustc_lint::LateContext<'tcx>, ty: &'tcx Ty<'tcx, AmbigArg>) {
        let TyKind::Path(QPath::Resolved(qself, path)) = &ty.kind else {
            return;
        };

        if qself.is_some() {
            return;
        }

        if should_lint_path(cx, path, ty.span) {
            lint_path(cx, ty.span);
        }
    }

    fn check_poly_trait_ref(
        &mut self,
        cx: &rustc_lint::LateContext<'tcx>,
        poly_trait_ref: &'tcx PolyTraitRef<'tcx>,
    ) {
        if should_lint_path(cx, poly_trait_ref.trait_ref.path, poly_trait_ref.trait_ref.path.span) {
            lint_path(cx, poly_trait_ref.trait_ref.path.span);
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
