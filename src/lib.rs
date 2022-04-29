#![feature(type_alias_impl_trait)]
type Ty<'a> = impl Fn() -> &'a str;
fn defining(s: &str) -> Ty<'_> { move || s }
fn execute(ty: Ty<'_>) -> &str { ty() }
