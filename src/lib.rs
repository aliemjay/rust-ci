#![feature(generic_associated_types)]
use std::marker::PhantomData;

trait AsyncFn<Arg> { type Output; }
trait RequestFamily { type Type<'a>; }
trait Service {}

struct MyFn;
impl AsyncFn<String> for MyFn { type Output = (); }

impl RequestFamily for String { type Type<'a> = String; }

struct ServiceFromAsyncFn<F, Req>(F, PhantomData<Req>);

impl<F, Req, O, O2> Service for ServiceFromAsyncFn<F, Req>
where
    Req: RequestFamily,
    //F: AsyncFn<Req>,
    //F: for<'a> AsyncFn<Req::Type<'a>, Output = O>,
    F: Fn(Req) -> O2,
    F: for<'a> Fn(Req::Type<'a>) -> O,
{
}

fn assert_service() -> impl Service {
    fn my_fn(_: String) {}
    ServiceFromAsyncFn(my_fn, PhantomData)
}
