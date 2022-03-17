use std::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

mod selector {
    use super::*;

    pub trait Selector<'a> {
        type Error;
        type Future: Future<Output = Self>;
        fn select() -> Self::Future;
    }

    impl<'a> Selector<'a> for u8 {
        type Error = ();
        type Future = Pin<Box<dyn Future<Output = Self>>>;
        fn select() -> Self::Future {
            unimplemented!()
        }
    }

    impl<'a, A, Err> Selector<'a> for (A,)
    where
        A: Selector<'a, Error = Err>,
    {
        type Error = ();
        type Future = CustomFut<'a, Err, A>;
        fn select() -> Self::Future {
            unimplemented!()
        }
    }

    pub struct CustomFut<'f, Err, A: Selector<'f, Error = Err>> {
        //ph: PhantomData<(&'f (), Req, Err, A, A::Error, A::Future)>,
        ph: PhantomData<(A::Future,)>,
    }

    impl<'f, Err, A: Selector<'f, Error = Err>> Future for CustomFut<'f, Err, A> {
        type Output = (A,);
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            unimplemented!()
        }
    }
}

mod async_fn {
    use super::*;

    pub trait AsyncFn {
        type Future: Future<Output = ()>;
        fn call(&self) -> Self::Future;
    }

    impl<F, Fut> AsyncFn for F
    where
        F: Fn() -> Fut,
        Fut: Future<Output = ()>,
    {
        type Future = Fut;
        fn call(&self) -> Self::Future {
            (self)()
        }
    }
}

async fn test() {
    use self::{async_fn::AsyncFn, selector::Selector};

    async fn upper<T: Selector<'static>>() {
        T::select().await;
    }

    async fn call_async_fn(inner: impl AsyncFn) {
        inner.call().await;
    }

    call_async_fn(upper::<(u8,)>).await;
}
