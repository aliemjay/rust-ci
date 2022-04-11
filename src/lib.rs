#![feature(generic_associated_types, type_alias_impl_trait)]
use core::future::Future;

trait Service<'a, Req> {
    type Future: Future;
    fn call(req: &'a Req) -> Self::Future;
}

impl<'a, Req> Service<'a, Req> for u8 {
    type Future = impl Future;
    fn call(req: &'a Req) -> Self::Future {
        async move { let x = req; }
    }
}

