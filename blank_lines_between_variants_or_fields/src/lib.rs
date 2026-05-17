#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_span;

use rustc_ast::{AttrStyle, Attribute, FieldDef, Item, ItemKind, Variant};
use rustc_lint::{EarlyContext, EarlyLintPass, LintContext};
use rustc_span::Span;

dylint_linting::declare_early_lint! {
    /// ### What it does
    ///
    /// Warns on blank lines between consecutive enum variants, struct fields,
    /// union fields, or variant fields.
    ///
    /// ### Why is this bad?
    ///
    /// Variants and fields belong to a single declaration and should be visually
    /// grouped. Blank lines inside a struct/enum body add vertical noise without
    /// communicating structure, since rustfmt already inserts a comma between
    /// every item. A blank line is only justified when separating logically
    /// independent sections — which is a sign the type should be split.
    ///
    /// Comments and doc comments between items are ignored: only physically
    /// blank lines trigger the lint.
    ///
    /// ### Example
    ///
    /// ```rust
    /// struct Config {
    ///     timeout: u64,
    ///
    ///     label: String,
    /// }
    ///
    /// enum Stage {
    ///     Start,
    ///
    ///     Middle,
    ///     End,
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// struct Config {
    ///     timeout: u64,
    ///     label: String,
    /// }
    ///
    /// enum Stage {
    ///     Start,
    ///     Middle,
    ///     End,
    /// }
    /// ```
    pub BLANK_LINES_BETWEEN_VARIANTS_OR_FIELDS,
    Warn,
    "blank lines between consecutive enum variants or struct fields"
}

fn first_outer_attr_span(attrs: &[Attribute]) -> Option<Span> {
    attrs
        .iter()
        .find(|attr| matches!(attr.style, AttrStyle::Outer))
        .map(|attr| attr.span)
}

fn logical_start(attrs: &[Attribute], item_span: Span) -> Span {
    first_outer_attr_span(attrs).unwrap_or(item_span)
}

fn snippet_has_blank_line(snippet: &str) -> bool {
    let lines: Vec<&str> = snippet.split('\n').collect();
    if lines.len() < 3 {
        return false;
    }
    lines[1..lines.len() - 1]
        .iter()
        .any(|line| line.trim().is_empty())
}

fn gap_has_blank_line(cx: &EarlyContext<'_>, prev: Span, next: Span) -> bool {
    let Ok(snippet) = cx.sess().source_map().span_to_snippet(prev.between(next)) else {
        return false;
    };
    snippet_has_blank_line(&snippet)
}

fn check_pairwise<T>(
    cx: &EarlyContext<'_>,
    items: &[T],
    get: impl Fn(&T) -> (Span, &[Attribute]),
) {
    for window in items.windows(2) {
        let (prev_span, _) = get(&window[0]);
        let (next_span, next_attrs) = get(&window[1]);
        if prev_span.from_expansion() || next_span.from_expansion() {
            continue;
        }
        let next_start = logical_start(next_attrs, next_span);
        if !gap_has_blank_line(cx, prev_span, next_start) {
            continue;
        }
        cx.span_lint(
            BLANK_LINES_BETWEEN_VARIANTS_OR_FIELDS,
            next_start,
            |diag| {
                diag.help(
                    "remove the blank line(s) above; variants and fields should be tightly packed",
                );
            },
        );
    }
}

fn check_fields(cx: &EarlyContext<'_>, fields: &[FieldDef]) {
    check_pairwise(cx, fields, |field| (field.span, field.attrs.as_slice()));
}

fn check_variants(cx: &EarlyContext<'_>, variants: &[Variant]) {
    check_pairwise(cx, variants, |variant| {
        (variant.span, variant.attrs.as_slice())
    });
    for variant in variants {
        check_fields(cx, variant.data.fields());
    }
}

impl EarlyLintPass for BlankLinesBetweenVariantsOrFields {
    fn check_item(&mut self, cx: &EarlyContext<'_>, item: &Item) {
        if item.span.from_expansion() {
            return;
        }
        match &item.kind {
            ItemKind::Enum(_, _, enum_def) => {
                check_variants(cx, &enum_def.variants);
            }
            ItemKind::Struct(_, _, variant_data) | ItemKind::Union(_, _, variant_data) => {
                check_fields(cx, variant_data.fields());
            }
            _ => {}
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
