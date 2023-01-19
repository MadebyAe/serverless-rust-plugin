use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, RequestExt, Response};
use serde_json::{json};

async fn member_two(request: Request) -> Response<Body> {
    let message = "Hello from member_two!";

    json!({ "message": message }).into_response().await
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func_closure = |request: Request| async move {
        Result::<Response<Body>, Error>::Ok(member_two(request).await)
    };

    run(service_fn(func_closure)).await?;
    Ok(())
}
