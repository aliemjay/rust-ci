#![feature(type_alias_impl_trait)]

type Fut<'a> = impl Sized; // call::<'a, 'empty>::closure#0

fn call<'a, 'b>(s: &'a str) -> Fut<'a>
where
    'b: 'a,
{
    move || { let s: &'a str = s; }
}

fn main() {}
