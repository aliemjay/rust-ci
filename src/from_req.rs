use core::{
    future::{ready, Future, Ready},
    marker::PhantomData,
    pin::Pin,
};

struct Req;

trait FromReq<'a> {
    type Out; // <-- may not be Self for borrowing extractors
    type Fut: Future<Output = Self::Out>;
    fn from_req(_: &'a Req) -> Self::Fut;
}

impl<'a, 'b> FromReq<'a> for &'b Req {
    type Out = &'a Req;
    type Fut = Ready<Self::Out>;
    fn from_req(req: &'a Req) -> Self::Fut {
        ready(req)
    }
}

impl<'a> FromReq<'a> for Req {
    type Out = Self;
    type Fut = Ready<Self>;
    fn from_req(_: &'a Req) -> Self::Fut {
        ready(Req)
    }
}

impl<'a, T1> FromReq<'a> for (T1,)
where
    T1: FromReq<'a>,
{
    type Out = (T1::Out,);
    type Fut = Pin<Box<dyn Future<Output = Self::Out> + 'a>>;
    fn from_req(req: &'a Req) -> Self::Fut {
        Box::pin(async move { (<T1 as FromReq<'a>>::from_req(req).await,) })
    }
}

impl<'a, T1, T2> FromReq<'a> for (T1, T2)
where
    T1: FromReq<'a>,
    T2: FromReq<'a>,
{
    type Out = (T1::Out, T2::Out);
    type Fut = Pin<Box<dyn Future<Output = Self::Out> + 'a>>;
    fn from_req(req: &'a Req) -> Self::Fut {
        Box::pin(async move {
            (
                <T1 as FromReq<'a>>::from_req(req).await,
                <T2 as FromReq<'a>>::from_req(req).await,
            )
        })
    }
}

trait FFn<A> {
    type Output;
    fn call(&self, args: A) -> Self::Output;
}

impl<A1, O, F: Fn(A1) -> O> FFn<(A1,)> for F {
    type Output = O;
    fn call(&self, (a1,): (A1,)) -> O {
        (self)(a1,)
    }
}

impl<A1, A2, O, F: Fn(A1, A2) -> O> FFn<(A1, A2,)> for F {
    type Output = O;
    fn call(&self, (a1, a2,): (A1, A2,)) -> O {
        (self)(a1, a2,)
    }
}

trait Handler<'a, T>: Clone + 'static {
    type Resp: 'static;
    type Fut: Future<Output = Self::Resp>;
    fn call(&'a self, req: &'a Req) -> Self::Fut;
}

impl<'a, T, F, Fut, Fut2, Resp> Handler<'a, T> for F
where
    F: FFn<T::Out, Output=Fut> + Clone + 'static,
    F: FFn<T, Output=Fut2>, // <-- for correctness and to assist type inference
    T: FromReq<'a>,
    Fut: Future<Output = Resp> + 'a,
    Resp: 'static,
{
    type Resp = Resp;
    type Fut = Pin<Box<dyn Future<Output = Resp> + 'a>>;
    fn call(&'a self, req: &'a Req) -> Self::Fut {
        Box::pin(async move { self.call(<T as FromReq<'a>>::from_req(req).await).await })
    }
}

struct HandlerService<H, T>(H, PhantomData<fn(T)>);

impl<H, T, R> HandlerService<H, T>
where
    for<'a> H: Handler<'a, T, Resp = R>,
{
    fn new(handler: H) -> Self {
        HandlerService(handler, PhantomData)
    }

    fn call(&self, req: Req) -> impl Future<Output = R> + 'static {
        let handler = self.0.clone();
        async move {
            let res = handler.call(&req).await;
            res
        }
    }
}

fn main() {
    async fn handler(_: (&Req, Req), _: &Req) -> String {
        String::from("hello")
    }

    let svc1 = HandlerService::new(handler);
    let _ = svc1.call(Req);

    #[derive(Clone)]
    struct Data;

    let data = Data;
    let _ = HandlerService::new(move |_: &Req| {
        let data = data.clone();
        async move { data }
    });
    
    let _ = <(&Req, &Req) as FromReq<'_>>::from_req(&Req);
}

