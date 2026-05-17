#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;

use rustc_ast::{Item, ItemKind, UseTree, UseTreeKind};
use rustc_lint::{EarlyContext, EarlyLintPass, LintContext};

dylint_linting::declare_early_lint! {
    /// ### What it does
    ///
    /// Warns on `use` statements that nest multi-segment paths inside a brace group.
    ///
    /// ### Why is this bad?
    ///
    /// Grouped imports such as `use module1::blah::{module2::bleep::Type, module3::Bleh};`
    /// or `use std::{collections::HashMap, path::PathBuf};` hide the modules being imported
    /// inside the group and make the import list hard to scan. Prefer one flat `use`
    /// statement per leaf path, reserving brace grouping for sibling leaf items such as
    /// `use std::collections::{HashMap, HashSet};`.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use module1::blah::{module2::bleep::Type, module3::Bleh};
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// use module1::blah::module2::bleep::Type;
    /// use module1::blah::module3::Bleh;
    /// ```
    pub NESTED_PATH_IN_USE_GROUP,
    Warn,
    "nested multi-segment paths inside a `use` group"
}

fn check_tree(cx: &EarlyContext<'_>, tree: &UseTree) {
    let UseTreeKind::Nested { items, .. } = &tree.kind else {
        return;
    };

    for (inner, _) in items {
        let nested_inside = matches!(inner.kind, UseTreeKind::Nested { .. });
        let multi_segment_prefix = inner.prefix.segments.len() >= 2;

        if multi_segment_prefix || nested_inside {
            cx.span_lint(NESTED_PATH_IN_USE_GROUP, inner.span, |diag| {
                diag.help(
                    "split this into separate `use` statements, one per leaf path, and reserve `{}` grouping for sibling leaf items",
                );
            });
        }

        check_tree(cx, inner);
    }
}

impl EarlyLintPass for NestedPathInUseGroup {
    fn check_item(&mut self, cx: &EarlyContext<'_>, item: &Item) {
        if item.span.from_expansion() {
            return;
        }
        if let ItemKind::Use(tree) = &item.kind {
            check_tree(cx, tree);
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
