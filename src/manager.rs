use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::{Mutex, RwLock};

type Middleware<V, R> = Box<dyn FnMut(V, Next<V, R>) -> BoxFuture<'static, R> + Send>;
type MiddlewareMutex<V, R> = Mutex<Middleware<V, R>>;
type ListOfMiddlewares<V, R> = Vec<MiddlewareMutex<V, R>>;
type SharableListOfMiddlewares<V, R> = Arc<RwLock<ListOfMiddlewares<V, R>>>;

pub struct Manager<V, R> {
    list: SharableListOfMiddlewares<V, R>,
}

impl<V: 'static, R: 'static> Manager<V, R> {
    /// Create new instance
    pub fn new() -> Self {
        Self {
            list: Arc::default(),
        }
    }

    pub fn last<M: 'static>(last: M) -> Self
    where
        M: FnMut(V, Next<V, R>) -> BoxFuture<'static, R> + Send,
    {
        let s = Self::new();
        s.next(last);

        s
    }

    /// Start processing the value
    pub async fn send(&self, value: V) -> R {
        let total = self.list.read().await.len();

        let qq = Arc::clone(&self.list);
        let next = Next {
            list: Arc::clone(&qq),
            next: total - 1,
        };

        let lock = self.list.read().await;
        let mut callback = lock.last().unwrap().lock().await;
        (callback)(value, next).await
    }

    pub fn next<M: 'static>(&self, m: M) -> &Self
    where
        M: FnMut(V, Next<V, R>) -> BoxFuture<'static, R> + Send,
    {
        let list = Arc::clone(&self.list);
        futures::executor::block_on(async move {
            let mut lock = list.write().await;
            lock.push(Mutex::new(Box::new(m)));
        });

        self
    }
}

pub struct Next<V, R> {
    list: SharableListOfMiddlewares<V, R>,
    next: usize,
}

impl<V, R> Next<V, R> {
    pub async fn call(mut self, value: V) -> R {
        let list = Arc::clone(&self.list);
        let lock = list.read().await;
        self.next -= 1;
        if let Some(next) = lock.get(self.next) {
            let mut callback = next.lock().await;
            return callback(value, self).await;
        }
        panic!("There must be a default")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    pub async fn test_last() {
        let result_str = "This is the end of the row";
        let manager = Manager::last(move |_v, _n| Box::pin(async move { result_str.to_string() }));

        assert_eq!(&manager.send(()).await, result_str);
    }

    #[tokio::test]
    pub async fn test_calling() {
        let manager = Manager {
            list: Arc::default(),
        };

        manager
            .next(|value, _next| Box::pin(async move { value }))
            .next(|value, next| Box::pin(async move { next.call(value * 2).await }))
            .next(|value, next| Box::pin(async move { next.call(value + 2).await }));

        let result: i32 = manager.send(10).await;

        assert_eq!(result, 24);
    }
}
