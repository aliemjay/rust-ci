#![feature(generic_associated_types)]
use std::marker::PhantomData as PhD;

pub trait Universe<'a>: 'static {
    type Ty;
}

impl<'a> Universe<'a> for &'static u8 {
    type Ty = &'a u8;
}

trait Service<Req> {}

struct BadCombinator<ReqU, S>(PhD<ReqU>, S);

impl<'c, ReqU, S> Service<<ReqU as Universe<'c>>::Ty> for BadCombinator<ReqU, S>
where
    ReqU: for<'a> Universe<'a>,
    S: Service<ReqU>,
    S: for<'a> Service<<ReqU as Universe<'a>>::Ty>,
{
}

fn test(f: impl for<'a> Service<&'a u8>) {
    fn assert_good(_: impl for<'a> Service<&'a u8>) {}

    // Without annotation
    // assert_good(BadCombinator(PhD::<&u8>, f));
    assert_good(BadCombinator(PhD, f));
}
mod jac {
trait Variable<'a> {
    type Type;
}

impl Variable<'_> for () {
    type Type = ();
}

fn check<F, T>(_: F)
where
    F: Fn(T), // <- if removed, all fn_* then require type annotations
    F: for<'a> Fn(<T as Variable<'a>>::Type),
    T: for<'a> Variable<'a>,
{
}

fn test(arg: impl Fn(())) {
    fn fn_1(_: ()) {}
    let fn_2 = |_: ()| ();
    let fn_3 = |a| fn_1(a);
    let fn_4 = arg;

    check(fn_1); // Error
    check(fn_2); // Ok
    check(fn_3); // Ok
    check(fn_4); // Error
}
}
