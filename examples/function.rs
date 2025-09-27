use simple_middleware::{Manager, Next};

#[tokio::main]
async fn main() {
    let manager = Manager::last(middleware1).await;

    manager.next(middleware2).await.next(middleware3).await;

    let result = manager.send(2).await;

    println!("result: {result}");
}

// 1. A middleware function must accept two arguments with the last
//    one being a `Next` type and must return a BoxFuture.
//
//    In this example, `Next` must take an i32 and return usize and this function
//    must return a BoxFuture that resolves to a usize
async fn middleware1(value: i32, _next: Next<i32, usize>) -> usize {
    (value * 4) as usize
}

async fn middleware2(value: i32, next: Next<i32, usize>) -> usize {
    next.call(value + 5).await
}

async fn middleware3(value: i32, next: Next<i32, usize>) -> usize {
    next.call(value * 6).await
}
