+++
title = "Integrating the Rust Axum Framework with Cloudflare Workers"
description = "Enhance Your Web Applications: Overcoming Challenges in Axum and Cloudflare Workers Integration: Discover the obstacles faced and solutions found while creating the axum-cloudflare-adapter, enabling seamless integration of Rust's Axum framework and Cloudflare workers."
date = 2023-04-20
+++

In this post, I'll share my experience integrating the Rust [Axum](https://github.com/tokio-rs/axum) framework into a
Cloudflare worker. The journey started with [contributing](https://github.com/tokio-rs/axum/pull/1382) to Axum, which
allowed it to compile to WebAssembly (Wasm). The next step was getting Axum to work in a Cloudflare worker, leading me
to create a crate called [axum-cloudflare-adapter](https://crates.io/crates/axum-cloudflare-adapter). Let's dive
into the challenges I faced and how I tackled them.

## Mapping Requests and Responses

Axum uses [http::Request](https://docs.rs/http/0.2.9/http/request/index.html) which is not the same as Cloudflare's [worker::Request](https://docs.rs/worker/latest/worker/struct.Request.html), so I needed to map them accordingly. I
created the `to_axum_request` function to solve this issue. The same problem exists with responses, so I created
the `to_worker_response` function.

```rust
use axum_cloudflare_adapter::{to_axum_request, to_worker_response};
use tower_service::Service;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let mut router: AxumRouter = AxumRouter::new();
    let axum_request = to_axum_request(req).await.unwrap();
    let axum_response = router.call(axum_request).await.unwrap();
    let response = to_worker_response(axum_response).await.unwrap();

    response
}
```

## Accessing Cloudflare's `worker::Env` within Axum Routes

To use Cloudflare's [worker::Env](https://docs.rs/worker/latest/worker/struct.Env.html) within my Axum routes, I initially tried to put it in the state, but `worker::Env`
doesn't implement `Sync` or `Send`. To work around this, I created `EnvWrapper`, which does implement `Sync` and `Send`
with no-ops. This is fine because workers are always executed in a single context.

```rust
use axum_cloudflare_adapter::{EnvWrapper};

#[derive(Clone)]
pub struct AxumState {
    pub env_wrapper: EnvWrapper,
}

#[event(fetch)]
pub async fn main(_: Request, env: Env, _: worker::Context) -> Result<Response> {
    let axum_state = AxumState {
        env_wrapper: EnvWrapper::new(env),
    };
}
```

## Handling Non-Send Futures in Axum Routes

Axum expects routes to return a `Send` future, but JS types don't implement `Send`. This is a problem if you
want to use `worker::Fetch` to make an HTTP request. To work around this issue, I created a macro that wraps the entire
Axum route in a `wasm_bindgen_futures::spawn_local` and passes the result of the route back to the main thread using
a `oneshot::channel`. Credit goes to [SebastiaanYN](https://github.com/SebastiaanYN) for the solution. I simply created a macro for it. To overcome
this issue, you can add the `#[worker_route_compat]` macro, and it'll make your Axum routes compatible.

```rust
#[worker_route_compat]
pub async fn index(State(state): State<AxumState>) -> impl IntoResponse {
    // your code
}

// The macro converts it to the following
pub async fn index(State(state): State<AxumState>) -> impl IntoResponse {
    wasm_bindgen_futures::spawn_local(async move {
        let result = {
            // your code
        };
        tx.send(result).unwrap();
    });
    rx.await.unwrap
}
```

I created a [demo](https://axum-cloudflare-adapter-example.logankeenan.workers.dev/) application using the
axum-cloudflare-adapter, [source](https://github.com/logankeenan/axum-cloudflare-adapter/tree/main/example). It proxies this blog through a worker.
