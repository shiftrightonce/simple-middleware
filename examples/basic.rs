use simple_middleware::Manager;

#[tokio::main]
async fn main() {
    // 1. Create an instance of the manager used to pass value and get result from the middlewares
    let m = Manager::new();

    // 2. This will be the last middleware to be called. Execution starts from the last middleware added to the collection.
    //    Note: this middleware is not calling the next one.
    m.next(|v, _n| Box::pin(async move { v + 1 })).await;

    // 3. In this example, this is the last middleware.
    //    It will be the first to be executed
    m.next(|value, next| {
        Box::pin(async move {
            // This is how you call the next middleware; by calling the `call` method on `next`
            next.call(value * 3).await
        })
    })
    .await;

    // 4. A value is being passed through the middlewares
    //    Note: We are also specifying the expected type that should be returned
    let ans: i32 = m.send(33).await;

    println!("ans: {ans}");
}
