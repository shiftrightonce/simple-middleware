 #Simple Middleware

 Provides the `middleware` pattern that can be used in various ways.
 Middleware pattern allows you to pass a payload through a collection of functions/methods
 expecting a specific result at the end.

 Each function has the capability to:
  - progress the execution to the next middleware
  - intercept the execution before calling the next middleware
  - post process the result returned from the previous middleware

 Below is a diagram of six middlewares where `last` is the destination.
 The execution starts at `m5` and progress to m4, m3 and so on.

```not_rust

                      Payload                        
                         |                           
                         v                           
 +--------------------------------------------------+
 |                     middleware #5                |
 |   +------------------------------------------+   |
 |   |                 middleware #4            |   |
 |   |   +---------------------------------+    |   |
 |   |   |             middleware #3       |    |   |
 |   |   |   +-------------------------+   |    |   |
 |   |   |   |         middleware #2   |   |    |   |
 |   |   |   |      +-----------+      |   |    |   |
 |   |   |   |      |   last    |      |   |    |   |
 |   |   |   |      +-----------+      |   |    |   |
 |   |   |   +-------------------------+   |    |   |
 |   |   +---------------------------------+    |   |
 |   +------------------------------------------+   |
 +------------------------+-------------------------+
                          |                          
                          |                          
                          v                          
                       Result                        
                             
```

 ## Basic example

```
 use simple_middleware::Manager;


 #[tokio::main]
 async fn main() {
     // 1. Create an instance of the manager used to pass value and get result from the middlewares
     let m = Manager::new();

     // 2. This will be the last middleware to be called. Execution starts from the last middleware added to the collection.
     //    Note: this middleware is not calling the next one.
     m.next(|v, _n| Box::pin(async move { v + 1 }));

     // 3. In this example, this is the last middleware.
     //    It will be the first to be executed
     m.next(|value, next| {
         Box::pin(async move {
             // This is how you call the next middleware; by calling the `call` method on `next`
             next.call(value * 3).await
         })
     });

     // 4. A value is being passed through the middlewares
     //    Note: We are also specifying the expected type that should be returned
     let ans: i32 = m.send(33).await;

     println!("ans: {ans}");
 }
```

## Examples

The [examples](https://github.com/shiftrightonce/orsomafo/tree/main/examples) folder contains simple and full examples. If none of the examples are helpful,
please reach out with your use case and I'll  try to provide one.

## Feedback

If you find this crate useful, please star the repository. Submit your issues and recommendations as well.

## License

### The MIT License (MIT)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.