// run-pass

pub trait FakeCoroutine {
    type Yield;
    type Return;
}

pub trait FakeFuture {
    type Output;
}

pub fn future_from_generator<
    T: FakeCoroutine<Yield = ()>
>(x: T) -> impl FakeFuture<Output = T::Return> {
    GenFuture(x)
}

struct GenFuture<T: FakeCoroutine<Yield = ()>>(#[allow(unused_tuple_struct_fields)] T);

impl<T: FakeCoroutine<Yield = ()>> FakeFuture for GenFuture<T> {
    type Output = T::Return;
}

fn main() {}
