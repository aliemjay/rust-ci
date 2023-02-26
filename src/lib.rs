#![feature(type_alias_impl_trait)]

use std::future::Future;

type FutNothing = impl Future<Output = ()>;

fn indirect() -> FutNothing {
    call(operation);
    operation()
}

async fn operation() {}

fn call(_: impl Fn() -> FutNothing) {}

fn main() {}
