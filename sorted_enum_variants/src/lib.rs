#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;

use rustc_ast::{Item, ItemKind, Variant};
use rustc_lint::{EarlyContext, EarlyLintPass, LintContext};

dylint_linting::declare_early_lint! {
    /// ### What it does
    ///
    /// Warns when an enum's variants are not declared in case-insensitive
    /// alphabetical order.
    ///
    /// ### Why is this bad?
    ///
    /// Alphabetical ordering makes variants easy to locate when scanning a
    /// large enum and keeps diffs minimal when new variants are added.
    ///
    /// Enums where ordering carries semantic meaning are excluded automatically:
    ///
    /// - any variant has an explicit discriminant (`Variant = 1`), since the
    ///   numeric order is the contract; or
    /// - the enum is `#[non_exhaustive]` and the user has tagged it
    ///   `#[allow(sorted_enum_variants)]`.
    ///
    /// For other meaningful-order enums (e.g. `LogLevel { Trace, Debug, Info,
    /// Warn, Error }`), silence the lint per-item with
    /// `#[allow(sorted_enum_variants)]`.
    ///
    /// ### Example
    ///
    /// ```rust
    /// enum Direction {
    ///     North,
    ///     East,
    ///     South,
    ///     West,
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// enum Direction {
    ///     East,
    ///     North,
    ///     South,
    ///     West,
    /// }
    /// ```
    pub SORTED_ENUM_VARIANTS,
    Warn,
    "enum variants should be declared in alphabetical order"
}

fn check_variants(cx: &EarlyContext<'_>, variants: &[Variant]) {
    if variants.iter().any(|variant| variant.disr_expr.is_some()) {
        return;
    }
    for window in variants.windows(2) {
        let prev = &window[0];
        let next = &window[1];
        if prev.span.from_expansion() || next.span.from_expansion() {
            continue;
        }
        let prev_key = prev.ident.name.as_str().to_ascii_lowercase();
        let next_key = next.ident.name.as_str().to_ascii_lowercase();
        if next_key < prev_key {
            let prev_name = prev.ident.name;
            let next_name = next.ident.name;
            cx.span_lint(SORTED_ENUM_VARIANTS, next.ident.span, |diag| {
                diag.help(format!(
                    "move `{next_name}` before `{prev_name}` to keep variants in alphabetical order",
                ));
            });
        }
    }
}

impl EarlyLintPass for SortedEnumVariants {
    fn check_item(&mut self, cx: &EarlyContext<'_>, item: &Item) {
        if item.span.from_expansion() {
            return;
        }
        if let ItemKind::Enum(_, _, enum_def) = &item.kind {
            check_variants(cx, &enum_def.variants);
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
