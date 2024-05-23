pub struct Middleware<V, R> {
    inner: fn(V) -> R,
}

impl<V, R> Middleware<V, R> {
    pub fn handle(&self, value: V) -> R {
        (self.inner)(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calling_middleware() {
        let middle = Middleware {
            inner: |val: i32| val + 2,
        };

        assert_eq!(middle.handle(50), 52, "Wrong answer. suppose to be 52");
    }
}
