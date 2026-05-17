#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_span;

use std::collections::{HashMap, HashSet};

use clippy_utils::str_utils::to_snake_case;

use rustc_ast::{
    AssocItemKind, AttrKind, Attribute, Crate, EnumDef, Item, ItemKind, MetaItemInner, ModKind,
    Ty, TyKind, Variant, VariantData, VisibilityKind,
};
use rustc_lint::{EarlyContext, EarlyLintPass, LintContext};
use rustc_span::Symbol;

dylint_linting::declare_pre_expansion_lint! {
    /// ### What it does
    ///
    /// Enforces the project's conventions on error types built with
    /// [`thiserror`](https://docs.rs/thiserror):
    ///
    /// 1. **Naming.** A `pub` error type must not be called literally `Error` —
    ///    it must carry a descriptive prefix (e.g. `ParseError`, `IoError`).
    /// 2. **Mandatory derive.** Any type whose name ends in `Error` must derive
    ///    `thiserror::Error` (`#[derive(Error)]` or `#[derive(thiserror::Error)]`).
    /// 3. **Constructor methods.** Each variant with one or more fields and
    ///    without a `#[from]` field attribute must have a snake-case
    ///    constructor `impl MyError { fn variant_name(...) -> Self }` somewhere
    ///    in the same crate.
    ///
    /// Alphabetical variant ordering is enforced separately by the
    /// `sorted_enum_variants` lint.
    ///
    /// ### Why is this bad?
    ///
    /// These conventions keep error types predictable: callers can `use` them
    /// without conflict, error rendering is uniform via `thiserror`, and
    /// complex variants have a single canonical constructor.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// use thiserror::Error;
    ///
    /// #[derive(Error, Debug)]
    /// pub enum Error {
    ///     #[error("bad")]
    ///     BadInput { reason: String },
    ///     #[error("io")]
    ///     A,
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// use thiserror::Error;
    ///
    /// #[derive(Error, Debug)]
    /// pub enum ParseError {
    ///     #[error("io")]
    ///     A,
    ///     #[error("bad")]
    ///     BadInput { reason: String },
    /// }
    ///
    /// impl ParseError {
    ///     pub fn bad_input(reason: impl Into<String>) -> Self {
    ///         Self::BadInput { reason: reason.into() }
    ///     }
    /// }
    /// ```
    pub THISERROR_CONVENTIONS,
    Warn,
    "enforce thiserror conventions: naming, mandatory derive, constructors for complex variants"
}

type InherentMethods = HashMap<Symbol, HashSet<Symbol>>;

fn self_ty_ident(ty: &Ty) -> Option<Symbol> {
    let TyKind::Path(_qself, path) = &ty.kind else {
        return None;
    };
    path.segments.last().map(|segment| segment.ident.name)
}

fn collect_inherent_methods(items: &[Box<Item>], out: &mut InherentMethods) {
    for item in items {
        match &item.kind {
            ItemKind::Impl(imp) => {
                if imp.of_trait.is_some() {
                    continue;
                }
                let Some(self_name) = self_ty_ident(&imp.self_ty) else {
                    continue;
                };
                let entry = out.entry(self_name).or_default();
                for assoc in &imp.items {
                    if !matches!(assoc.kind, AssocItemKind::Fn(_)) {
                        continue;
                    }
                    if let Some(ident) = assoc.kind.ident() {
                        entry.insert(ident.name);
                    }
                }
            }
            ItemKind::Mod(_, _, ModKind::Loaded(inner, _, _)) => {
                collect_inherent_methods(inner, out);
            }
            _ => {}
        }
    }
}

fn attr_is_derive_target(meta: &MetaItemInner, expected: &[&str]) -> bool {
    let MetaItemInner::MetaItem(item) = meta else {
        return false;
    };
    let segments = &item.path.segments;
    if segments.len() != expected.len() {
        return false;
    }
    segments
        .iter()
        .zip(expected.iter())
        .all(|(segment, name)| segment.ident.name.as_str() == *name)
}

fn attr_lists_thiserror_error(attr: &Attribute) -> bool {
    let AttrKind::Normal(normal) = &attr.kind else {
        return false;
    };
    let segments = &normal.item.path.segments;
    if !matches!(segments.as_slice(), [seg] if seg.ident.name.as_str() == "derive") {
        return false;
    }
    let Some(list) = attr.meta_item_list() else {
        return false;
    };
    list.iter()
        .any(|inner| attr_is_derive_target(inner, &["Error"]) || attr_is_derive_target(inner, &["thiserror", "Error"]))
}

fn derives_thiserror_error(attrs: &[Attribute]) -> bool {
    attrs.iter().any(attr_lists_thiserror_error)
}

fn field_has_from_attr(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        let AttrKind::Normal(normal) = &attr.kind else {
            return false;
        };
        matches!(normal.item.path.segments.as_slice(), [seg] if seg.ident.name.as_str() == "from")
    })
}

fn variant_has_from(variant: &Variant) -> bool {
    match &variant.data {
        VariantData::Struct { fields, .. } | VariantData::Tuple(fields, _) => {
            fields.iter().any(|field| field_has_from_attr(&field.attrs))
        }
        VariantData::Unit(_) => false,
    }
}

fn variant_is_complex(variant: &Variant) -> bool {
    match &variant.data {
        VariantData::Struct { fields, .. } | VariantData::Tuple(fields, _) => !fields.is_empty(),
        VariantData::Unit(_) => false,
    }
}

fn check_naming(cx: &EarlyContext<'_>, item: &Item) {
    let Some(ident) = item.kind.ident() else {
        return;
    };
    if !matches!(item.vis.kind, VisibilityKind::Public) {
        return;
    }
    if ident.name.as_str() != "Error" {
        return;
    }
    if !derives_thiserror_error(&item.attrs) {
        return;
    }
    cx.span_lint(THISERROR_CONVENTIONS, ident.span, |diag| {
        diag.help(
            "prefix the error type with a descriptive name (e.g. `ParseError`, `IoError`) so callers can import it without conflicting with `std::error::Error` or other crates' `Error` types",
        );
    });
}

fn check_must_derive_thiserror(cx: &EarlyContext<'_>, item: &Item) {
    let Some(ident) = item.kind.ident() else {
        return;
    };
    if !matches!(
        item.kind,
        ItemKind::Enum(..) | ItemKind::Struct(..) | ItemKind::Union(..)
    ) {
        return;
    }
    if !ident.name.as_str().ends_with("Error") {
        return;
    }
    if derives_thiserror_error(&item.attrs) {
        return;
    }
    cx.span_lint(THISERROR_CONVENTIONS, ident.span, |diag| {
        diag.help(
            "this type's name ends in `Error`; derive `thiserror::Error` on it (`#[derive(Error)]` or `#[derive(thiserror::Error)]`) so error rendering and trait impls are consistent across the codebase",
        );
    });
}

fn check_variant_constructors(
    cx: &EarlyContext<'_>,
    enum_name: Symbol,
    enum_def: &EnumDef,
    inherent_methods: &InherentMethods,
) {
    let methods = inherent_methods.get(&enum_name);
    for variant in &enum_def.variants {
        if !variant_is_complex(variant) {
            continue;
        }
        if variant_has_from(variant) {
            continue;
        }
        let expected = to_snake_case(variant.ident.name.as_str());
        let has_constructor = methods
            .map(|set| set.contains(&Symbol::intern(&expected)))
            .unwrap_or(false);
        if has_constructor {
            continue;
        }
        let variant_name = variant.ident.name;
        cx.span_lint(THISERROR_CONVENTIONS, variant.ident.span, |diag| {
            diag.help(format!(
                "add a constructor on `{enum_name}`: `fn {expected}(...) -> Self {{ Self::{variant_name} {{ ... }} }}` so callers don't have to spell out the variant payload",
            ));
        });
    }
}

fn check_items(cx: &EarlyContext<'_>, items: &[Box<Item>], inherent_methods: &InherentMethods) {
    for item in items {
        if item.span.from_expansion() {
            continue;
        }

        check_naming(cx, item);
        check_must_derive_thiserror(cx, item);

        if let ItemKind::Enum(ident, _, enum_def) = &item.kind
            && derives_thiserror_error(&item.attrs)
        {
            check_variant_constructors(cx, ident.name, enum_def, inherent_methods);
        }

        if let ItemKind::Mod(_, _, ModKind::Loaded(inner, _, _)) = &item.kind {
            check_items(cx, inner, inherent_methods);
        }
    }
}

impl EarlyLintPass for ThiserrorConventions {
    fn check_crate(&mut self, cx: &EarlyContext<'_>, krate: &Crate) {
        let mut inherent_methods = InherentMethods::new();
        collect_inherent_methods(&krate.items, &mut inherent_methods);
        check_items(cx, &krate.items, &inherent_methods);
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test_example(env!("CARGO_PKG_NAME"), "main");
}
