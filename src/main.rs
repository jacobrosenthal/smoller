use futures::prelude::*;

fn block<F: Future>(mut f: F) -> F::Output {
    let waker = waker();
    let mut future = unsafe { std::pin::Pin::new_unchecked(&mut f) };
    let mut context = &mut futures::task::Context::from_waker(&waker);

    loop {
        //todo Poll::Pending
        if let futures::task::Poll::Ready(x) = future.as_mut().poll(&mut context) {
            return x;
        }
    }
}

async fn one() -> u32 {
    1
}

fn main() {
    let res = block(one());

    dbg!(res);
}

// just bullshit the waker
// stolen from japaric https://github.com/japaric/embedded2020/blob/99bc6b1a83aa9b089a03c5836ee132392bd7836f/firmware/executor/src/lib.rs
pub fn waker() -> futures::task::Waker {
    unsafe fn clone(_: *const ()) -> futures::task::RawWaker {
        loop {
            continue;
        }
    }

    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}

    static VTABLE: futures::task::RawWakerVTable =
        futures::task::RawWakerVTable::new(clone, wake, wake_by_ref, drop);

    unsafe { futures::task::Waker::from_raw(futures::task::RawWaker::new(&(), &VTABLE)) }
}
