#![no_implicit_prelude]
#![feature(
    arbitrary_self_types,
    async_await,
    await_macro,
    futures_api,
    generator_trait,
    generators,
    pin
)]

// This is using no_implicit_prelude to test that the macros don't accidentally
// refer directly to any paths from core's implicitly injected prelude and
// instead everything is going through the internal re-export.

#[test]
fn smoke() {
    let future = async {
        ::std::await!(::embrio_async::async_block!(foo, {
            ::embrio_async::await!(foo, async { 5 })
        }))
    };
    ::std::assert_eq!(::futures::executor::block_on(future), 5);
}