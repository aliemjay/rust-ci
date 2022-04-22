#![feature(generic_associated_types)]
use std::marker::PhantomData as PhD;

pub trait Universe: 'static {
    type Ty<'a>;
}

impl Universe for &'static u8 {
    type Ty<'a> = &'a u8;
}

trait Service<Req> {}

struct BadCombinator<ReqU, S>(PhD<ReqU>, S);

impl<ReqU, S> Service<ReqU::Ty<'_>> for BadCombinator<ReqU, S>
where
    ReqU: Universe,
    S: Service<ReqU>,
    S: for<'a> Service<ReqU::Ty<'a>>,
{
}

fn test(f: impl for<'a> Service<&'a u8>) {
    fn assert_good(_: impl for<'a> Service<&'a u8>) {}

    // Without annotation
    // assert_good(BadCombinator(PhD::<&u8>, f));
    assert_good(BadCombinator(PhD, f));
}
