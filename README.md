# tokio-shared-rt

Allow `#[tokio::test]` to use a *shared* runtime, so that static variables that connect to a tokio runtime inside are still valid between different test cases.

For example, if you have a global, static database connection pool, it internally holds references to some TCP connections which are bound to the runtime that created it. So if you have two test cases that are both marked `#[tokio::test]` and access that db pool, good chance is that one of them will fail.
```
thread 't3' panicked at 'called `Result::unwrap()` on an `Err` value: Custom { kind: Other, error: "A Tokio 1.x context was found, but it is being shutdown." }'
```

This crate provides a macro that uses a *shared tokio runtime* to run test cases to avoid this problem. Just replace your `#[tokio::test]` with `#[tokio_shared_rt::test(shared)]` and vualá! Now test passes.

```
#[tokio_shared_rt::test(shared)]
async fn t1() {
    db_pool().foo().await;
}
#[tokio_shared_rt::test(shared)]
async fn t2() {
    db_pool().foo().await;
}
```
