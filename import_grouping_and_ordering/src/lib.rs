#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_span;

use rustc_ast::{Crate, Item, ItemKind, ModKind, UseTree};
use rustc_lint::{EarlyContext, EarlyLintPass, LintContext};
use rustc_span::Span;
use rustc_span::symbol::kw;

dylint_linting::declare_early_lint! {
    /// ### What it does
    ///
    /// Warns when `use` declarations in a module are not grouped and ordered as
    /// 1) standard library, 2) external crates, 3) same crate, with a blank line
    /// separating each group.
    ///
    /// ### Why is this bad?
    ///
    /// Consistently grouping `use` declarations makes the imports section of a file
    /// easy to scan. Mixing standard library, external crate, and same-crate imports
    /// without separation hides relationships between modules.
    ///
    /// The expected layout is:
    ///
    /// 1. `std`, `core`, `alloc`
    /// 2. external crates (anything else)
    /// 3. same-crate paths (`crate::`, `self::`, `super::`)
    ///
    /// with a blank line between each non-empty group. Equivalent to
    /// `rustfmt --config group_imports=StdExternalCrate,imports_granularity=Module`.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// use serde::Serialize;
    /// use std::collections::HashMap;
    /// use crate::models::DataModel;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// use std::collections::HashMap;
    ///
    /// use serde::Serialize;
    ///
    /// use crate::models::DataModel;
    /// ```
    pub IMPORT_GROUPING_AND_ORDERING,
    Warn,
    "`use` declarations not grouped into stdlib / external / same-crate order with blank-line separators"
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Group {
    Stdlib,
    External,
    SameCrate,
}

impl Group {
    fn label(self) -> &'static str {
        match self {
            Self::Stdlib => "standard library",
            Self::External => "external crate",
            Self::SameCrate => "same-crate",
        }
    }
}

fn classify(tree: &UseTree) -> Group {
    let first_named = tree
        .prefix
        .segments
        .iter()
        .find(|s| s.ident.name != kw::PathRoot);

    let Some(segment) = first_named else {
        return Group::External;
    };

    match segment.ident.name.as_str() {
        "std" | "core" | "alloc" => Group::Stdlib,
        "crate" | "self" | "super" => Group::SameCrate,
        _ => Group::External,
    }
}

fn line_of(cx: &EarlyContext<'_>, span: Span) -> usize {
    cx.sess().source_map().lookup_char_pos(span.lo()).line
}

fn end_line_of(cx: &EarlyContext<'_>, span: Span) -> usize {
    cx.sess().source_map().lookup_char_pos(span.hi()).line
}

fn check_items(cx: &EarlyContext<'_>, items: &[Box<Item>]) {
    let leading_uses: Vec<&Item> = items
        .iter()
        .map(|i| &**i)
        .take_while(|i| matches!(i.kind, ItemKind::Use(_) | ItemKind::ExternCrate(..)))
        .filter(|i| matches!(i.kind, ItemKind::Use(_)) && !i.span.from_expansion())
        .collect();

    if leading_uses.len() < 2 {
        return;
    }

    let entries: Vec<(Group, &Item)> = leading_uses
        .iter()
        .map(|item| {
            let ItemKind::Use(tree) = &item.kind else {
                unreachable!("filtered above");
            };
            (classify(tree), *item)
        })
        .collect();

    let mut max_seen = entries[0].0;
    for (group, item) in &entries[1..] {
        if *group < max_seen {
            cx.span_lint(IMPORT_GROUPING_AND_ORDERING, item.span, |diag| {
                diag.help(format!(
                    "place this {} import above the {} imports (order: standard library, external crates, same-crate)",
                    group.label(),
                    max_seen.label()
                ));
            });
        } else {
            max_seen = *group;
        }
    }

    for window in entries.windows(2) {
        let (prev_group, prev_item) = window[0];
        let (next_group, next_item) = window[1];

        if prev_group == next_group {
            continue;
        }
        if next_group < prev_group {
            continue;
        }

        let prev_end = end_line_of(cx, prev_item.span);
        let next_start = line_of(cx, next_item.span);

        if next_start <= prev_end + 1 {
            cx.span_lint(IMPORT_GROUPING_AND_ORDERING, next_item.span, |diag| {
                diag.help(format!(
                    "insert a blank line before this {} import to separate it from the {} group",
                    next_group.label(),
                    prev_group.label()
                ));
            });
        }
    }
}

impl EarlyLintPass for ImportGroupingAndOrdering {
    fn check_crate(&mut self, cx: &EarlyContext<'_>, krate: &Crate) {
        check_items(cx, &krate.items);
    }

    fn check_item(&mut self, cx: &EarlyContext<'_>, item: &Item) {
        if item.span.from_expansion() {
            return;
        }
        if let ItemKind::Mod(_, _, ModKind::Loaded(items, _, _)) = &item.kind {
            check_items(cx, items);
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
