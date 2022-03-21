use core::{future::Future, marker::PhantomData, pin::Pin};


trait AsyncClosure<Args> {
    type Output;
    type Future: Future<Output = Self::Output>;

    fn call(&self, arg: Args) -> Self::Future;
}

impl<F, Arg1, Arg2, Fut> AsyncClosure<(Arg1, Arg2)> for F
where
    F: Fn(Arg1, Arg2) -> Fut,
    Fut: Future,
{
    type Output = Fut::Output;
    type Future = Fut;

    fn call(&self, (arg1, arg2): (Arg1, Arg2)) -> Self::Future {
        (self)(arg1, arg2)
    }
}

//#[cfg(local_fact)]
trait ServiceFactory<Req, Arg = ()> {
    /// Responses given by the created services.
    type Response;

    /// Errors produced by the created services.
    type Error;

    /// The kind of `Service` created by this factory.
    type Service;

    /// The future of the `Service` instance.g
    type Future: Future<Output = Result<Self::Service, Self::Error>>;

    /// Create and return a new service asynchronously.
    fn new_service(&self, arg: Arg) -> Self::Future;
}


trait ServiceFactoryExt<Req, Arg>: ServiceFactory<Req, Arg> {
    fn enclosed_fn<T>(self, transform: T) -> Wrapper<Self, T>
    where
        //T: for<'s> AsyncClosure<(&'s Self::Service, Req)> + Clone,
        //Self: ServiceFactory<Req, Arg> + Sized,
        Self: Sized,
    {
        Wrapper(self, transform)
    }
}

impl<F, Req, Arg> ServiceFactoryExt<Req, Arg> for F where F: ServiceFactory<Req, Arg> {}

struct Wrapper<SF, T>(SF, T);

impl<SF, Req, Arg, T, Res, Err> ServiceFactory<Req, Arg> for Wrapper<SF, T>
where
    SF: ServiceFactory<Req, Arg, Error = Err>,
    T: for<'s> AsyncClosure<(&'s SF::Service, Req), Output = Result<Res, Err>> + Clone,
    //Err: From<SF::Error>,
{
    type Response = Res;
    type Error = Err;
    type Service = Id;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Service, Self::Error>>>>;

    fn new_service(&self, arg: Arg) -> Self::Future { loop {} }
}

struct Id;

impl<Arg> ServiceFactory<(), Arg> for Id {
    type Response = ();
    type Error = ();
    type Service = Id;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Service, Self::Error>>>>;

    fn new_service(&self, arg: Arg) -> Self::Future {
        loop{}
    }
}

async fn index(s: ()) -> Result<(), ()> {
    loop {}
}

async fn enclosed_fn() {
    fn enclosed_2<S>(service: S, req: ()) -> impl Future<Output = Result<(), ()>> + 'static
    where
        //S: Service<(), Response = (), Error = ()>,
    {
        async { loop {} }
    }

    fn gat<Fut>(f: impl for<'a> Fn(&'a str, ()) -> Fut) {}
    //gat(enclosed_2);

    async fn enclosed<S>(service: &S, req: ()) -> Result<(), ()>
    where
        //S: Service<(), Response = (), Error = ()>,
    {
        loop {}
    }

    #[cfg(old_methods)]
    let fact = fn_service(index)
        .enclosed_fn(enclosed)
        .enclosed_fn(enclosed)
        .enclosed_fn(enclosed)
        .enclosed_fn(enclosed)
        .enclosed_fn(enclosed)
        .enclosed_fn(enclosed)
        .enclosed_fn(enclosed)
        .enclosed_fn(enclosed)
        .enclosed_fn(enclosed)
        .enclosed_fn(enclosed)
        .enclosed_fn(enclosed)
        .enclosed_fn(enclosed);
    //return fact.new_service(()).await.unwrap();

    //let fact = fn_service(index);
    let fact = Id;
    let fact = Wrapper(fact, enclosed);
    let fact = Wrapper(fact, enclosed);
    let fact = Wrapper(fact, enclosed);
    let fact = Wrapper(fact, enclosed);
    let fact = Wrapper(fact, enclosed);
    let fact = Wrapper(fact, enclosed);
    let fact = Wrapper(fact, enclosed);
    let fact = Wrapper(fact, enclosed);
    let fact = Wrapper(fact, enclosed);
    //let fact = PipelineServiceFactory::<_, _, marker::EnclosedFn>::new(fact, enclosed);
    //let fact = PipelineServiceFactory::<_, _, marker::EnclosedFn>::new(fact, enclosed);
    //let fact = PipelineServiceFactory::<_, _, marker::EnclosedFn>::new(fact, enclosed);
    //let fact = PipelineServiceFactory::<_, _, marker::EnclosedFn>::new(fact, enclosed);

    //fact
    //fact.new_service(()).await.unwrap();
    //fact.new_service(()).await.unwrap();
    fact.new_service(()).await.unwrap();
    ServiceFactory::<_, _>::new_service(&fact, ()).await.unwrap();
    //fn impl_factory(_: impl ServiceFactory<(), ()>) {}
    //impl_factory(fact);
    /*
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap();
    ServiceFactory::<&'static str, ()>::new_service(&fact, ()).await.unwrap()
    */

    //assert_eq!(res, "251");
}

fn main() {
    let _ = enclosed_fn();
    println!("Hello, world!");
}
