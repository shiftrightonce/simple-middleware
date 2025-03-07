use simple_middleware::Manager;

#[tokio::main]
async fn main() {
    // 1. Using the static method `last`, we can create a new manager and pass the last middleware
    //    in one step
    let manager = Manager::last(|v, _n| Box::pin(async move { v + 22_usize })).await;

    // 2. We can also chain calls to the `next` method
    manager
        .next(|mut v, next| {
            Box::pin(async move {
                v += 4;
                next.call(v).await
            })
        })
        .await
        .next(|v, next| Box::pin(async move { next.call(v * 2).await }))
        .await;

    let result: usize = manager.send(200).await;

    println!("result: {result}");
}
