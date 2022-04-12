#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

trait Service<'a, Req> {
    type Output where Req: 'a;
    fn call(req: &'a Req) -> Self::Output;
}

impl<'a, 'r, Req> Service<'a, &'r Req> for u8 {
    type Output = &'a &'r Req where 'r: 'a;
    fn call(req: &'a &'r Req) -> Self::Output {
        req
    }
}

impl<'a, 'r, Req> Service<'a, &'r Req> for u16 {
    type Output = impl Copy where 'r: 'a;
    fn call(req: &'a &'r Req) -> Self::Output {
        req
    }
}
