# Lack of test isolation

One unfortunate thing about how the `wasm-bindgen` test runner works is that it appears not to isolate the environment in which individual tests are run.

This potentially causes some weirdness with the `Parser::init()` function, which must be called before the `tree-sitter` library is used.

What happens is that each individual test calls `Parser::init()` at the beginning (via `crate::util::parser::init()`), which is fine, but if you go through several of the tests and remove those calls, surprisingly the tests still pass.

This is because the call to `Parser::init()` that actually did something occurred in an earlier test. You can verify this by removing *all* of the calls to `Parser::init()` from *all* of the tests, and then see that almost all of the tests begin to fail (whereas removing several, but not all calls, still potentially allows all tests to pass, depending on execution order).

It appears that in practice, calling `Parser::init()` multiple times seems fine, but it's good to be aware of the fact that this could potentially cause problems at some point in the future.

Also see: https://github.com/rustwasm/wasm-bindgen/pull/2831
