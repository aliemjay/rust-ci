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
    S: for<'a> Service<<ReqU as Universe<'a>>::Ty>,
    S: Service<ReqU>,
{
}

fn test(f: impl for<'a> Service<&'a u8>) {
    fn assert_good(_: impl for<'a> Service<&'a u8>) {}

    // Without annotation
    // assert_good(BadCombinator(PhD::<&u8>, f));
    assert_good(BadCombinator(PhD::<&u8>, f));
}
