#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_middle;

use rustc_hir::{Item, ItemKind, VariantData};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_middle::ty::Visibility;

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Warns on `pub` fields of named structs, tuple structs, and unions.
    ///
    /// ### Why is this bad?
    ///
    /// Exposing fields directly makes a type's invariants impossible to enforce and ties
    /// callers to the field layout. Prefer the project's accessor style so the
    /// representation can change without breaking downstream code:
    ///
    /// - getter: `fn timeout(&self) -> …`
    /// - mutator: `fn set_timeout(&mut self, …)`
    /// - builder: `fn with_timeout(mut self, …) -> Self` — should be written in terms of
    ///   `set_timeout` so that any invariants enforced by the mutator are also applied
    ///   when building.
    ///
    /// For newtype wrappers, expose an `into_inner` / accessor pair rather than `pub`
    /// on the positional field. `#[repr(C)]` FFI structs and similar cases where public
    /// fields are intentional can opt out with `#[allow(pub_field_in_struct)]`.
    ///
    /// ### Example
    ///
    /// ```rust
    /// pub struct Config {
    ///     pub timeout: u64,
    /// }
    ///
    /// pub struct Wrapper(pub Inner);
    /// pub struct Inner;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// pub struct Config {
    ///     timeout: u64,
    /// }
    ///
    /// impl Config {
    ///     pub fn timeout(&self) -> u64 { self.timeout }
    ///     pub fn set_timeout(&mut self, value: u64) { self.timeout = value; }
    ///     pub fn with_timeout(mut self, value: u64) -> Self {
    ///         self.set_timeout(value);
    ///         self
    ///     }
    /// }
    ///
    /// pub struct Wrapper(Inner);
    /// pub struct Inner;
    ///
    /// impl Wrapper {
    ///     pub fn inner(&self) -> &Inner { &self.0 }
    ///     pub fn into_inner(self) -> Inner { self.0 }
    /// }
    /// ```
    pub PUB_FIELD_IN_STRUCT,
    Warn,
    "public fields on a struct or union; prefer accessors and mutators"
}

fn check_variant_data<'tcx>(
    cx: &LateContext<'tcx>,
    container_kind: &'static str,
    variant_data: &'tcx VariantData<'tcx>,
) {
    for field in variant_data.fields() {
        if field.span.from_expansion() {
            continue;
        }

        if !matches!(cx.tcx.visibility(field.def_id), Visibility::Public) {
            continue;
        }

        let span = if field.vis_span.is_empty() {
            field.span
        } else {
            field.vis_span
        };

        let positional = field.is_positional();

        cx.span_lint(PUB_FIELD_IN_STRUCT, span, |diag| {
            if positional {
                diag.help(format!(
                    "the positional field on this {container_kind} is public; expose it through an accessor pair (e.g. `fn inner(&self) -> &T`, `fn into_inner(self) -> T`) or restrict visibility to `pub(crate)`"
                ));
            } else {
                diag.help(format!(
                    "this {container_kind} field is public; expose it through the project's accessor style: `fn {name}(&self) -> …`, `fn set_{name}(&mut self, …)`, `fn with_{name}(mut self, …) -> Self` (delegating to `set_{name}`), or restrict visibility to `pub(crate)`",
                    name = field.ident.name
                ));
            }
        });
    }
}

impl<'tcx> LateLintPass<'tcx> for PubFieldInStruct {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        if item.span.from_expansion() {
            return;
        }

        match &item.kind {
            ItemKind::Struct(_, _, variant_data) => {
                check_variant_data(cx, "struct", variant_data);
            }
            ItemKind::Union(_, _, variant_data) => {
                check_variant_data(cx, "union", variant_data);
            }
            _ => {}
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
