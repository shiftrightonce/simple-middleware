use simple_middleware::Manager;

#[tokio::main]
async fn main() {
    // 1. The last/inner middleware will "create" a new user
    let manager = Manager::last(|payload: Payload, _| {
        Box::pin(async move {
            let id = std::time::SystemTime::now();
            let user = User {
                id: format!("{:?}", id),
                username: format!("{}{}", &payload.email, &payload.age),
            };

            UserCreatedResult {
                success: true,
                message: "User created successfully".to_string(),
                user: Some(user),
            }
        })
    });

    // 2. This middleware checks the new user payload, verifying their age
    //    before passing the payload on to the next middleware. In this case
    //    it will be the middleware the creates/inserts the user.
    manager.next(|payload, next| {
        Box::pin(async move {
            // validate the data before calling the next middleware
            if payload.age < 18 {
                return UserCreatedResult {
                    success: false,
                    message: "User is under 18".to_string(),
                    user: None,
                };
            }

            next.call(payload).await
        })
    });

    // 3. Fake a failed process
    let fail_result = manager
        .send(Payload {
            email: "foo@example.com".to_string(),
            age: 10,
        })
        .await;

    println!(
        "user created: {}, message: {}, {:?}",
        &fail_result.success, &fail_result.message, &fail_result.user
    );

    // 4. Fake a successful process
    let success_result = manager
        .send(Payload {
            email: "foo@example.com".to_string(),
            age: 25,
        })
        .await;

    println!(
        "user created: {}, message: {}, user: {:?}",
        &success_result.success, &success_result.message, &success_result.user
    );
}

struct Payload {
    email: String,
    age: u16,
}

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    pub id: String,
    pub username: String,
}

struct UserCreatedResult {
    success: bool,
    message: String,
    user: Option<User>,
}
