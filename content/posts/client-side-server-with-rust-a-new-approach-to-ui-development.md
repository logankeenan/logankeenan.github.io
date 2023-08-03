+++
title = "Client-Side Server with Rust: A New Approach to UI Development"
description = "Explore how to build a client-side server using Rust and WebAssembly, offering a unique UI development alternative. This guide demonstrates compiling Rust to run in the browser, enhancing performance and user experience. Ideal for developers interested in innovative web solutions, Rust programming, and WebAssembly."
date = 2023-08-02
+++

This blog post demonstrates how to leverage the simplicity of server-side patterns while enjoying the benefits of a
client-side application, by using a Rust server on the client.

## The Current State

In a server-side application, HTML is delivered upon a page request. User interactions such as form
submissions or link clicks prompt HTTP requests to the app, which then responds with HTML. This methodology has the
advantage of a fast initial page load and arguably more straightforward development. However, the incurred latency from
continuous server round-trips can degrade the user experience.

On the other hand, a client-side application results in an improved user experience with
snappier interactions due to the in-browser dynamic updates to the page. However, there are a few cons. The initial page
load can be slow, performance implications executing lots of JavaScript, and the overall complexity is arguably higher.

## A Server on the Client?

The idea of running a server on the client might sound unusual. In essence, it means compiling a Rust server into
WebAssembly, which can then be called directly in the browser. My
Previous [solution](https://logankeenan.com/posts/a-rust-server-app-compiled-to-wasm-as-an-spa/) had its
drawbacks, event hijacking, DOM updates, JS integration, and performance concerns. Then I came across Richard
Anaya's brilliant Wasm Service Worker [POC](https://github.com/richardanaya/wasm-service).

Service workers, often utilized in offline web apps, intercept HTTP requests and cache responses. Instead of making new
HTTP requests, subsequent requests access the cached response. In our case, instead of making an HTTP
request to the network, we're going to call our Wasm app with the request, and return the response. The browser doesn't
know any different and thinks it's communicating with a remote server.

This approach brings several advantages:

- Enhances the user experience as code is executed client-side.
- Boosts performance since operations run on a different thread.
- Arguably simplifies development as you only have to manage the server app.

Additionally, we experience faster initial page loads because the same Rust app server code can be used to render the
initial page on the server. This means the user can interact with the page immediately and even before the client-side
app has loaded, and if the client-side app still hasn't loaded, it will default to server-side rendering.

How can I make this interactive? Anchor tags and forms can go a long way, but for more
dynamic interactions, you can do whatever you want. Integrate htmx, load your favorite JS framework, or go wild and use
some vanilla JS. The browser thinks it's communicating with a server and rendering HTML, so you're not limited.

## The Proof of Concept

I'm going to demonstrate this idea with Rust using the [Axum](https://github.com/tokio-rs/axum) framework, however, this
could be done with any server framework
that will compile to Wasm and called with an HTTP request. I created a basic note-taking app, it allows for create,
read, update, and search. Check out the [Source Code & Demos](#source-code-demos)

I created a Rust library which exposes a function called `create_app`. It creates the Axum router with routes, this
encapsulates the entire app. A consumer of the library can create the app and pass it an HTTP request to receive an HTTP
response. The `create_app` function allows the app be integrated in various platforms.

```rust
pub fn create_app() -> Router {
    let router: AxumRouter = AxumRouter::new()
        .route("/", get(index))
        .route("/create", post(create_note))
        .route("/update", post(update_note))
        .route("/show/:id", get(show_note))
        .route("/edit/:id", get(edit_note))
        .route("/search", get(search_note))
        .layer(middleware::from_fn(set_user_id_cookie));

    router
}
```

Browser integration is done through a Rust library which will compile to Wasm and integrate with JavaScript
using [wasm_bindgen](https://github.com/rustwasm/wasm-bindgen). The `app` function is called with a `wasm_request`, the
`wasm_request` is converted to an Axum compatible request, the router is called with the request, the response is
converted back to a `wasm_response`, and returned from the function to JavaScript. I
created [axum-browser-adapter](https://github.com/logankeenan/axum-browser-adapter) to make this integration easier.

```rust 
pub use axum_browser_adapter::WasmRequest;

#[wasm_bindgen]
pub async fn app(wasm_request: WasmRequest) -> WasmResponse {
    let mut router = create_app();
    let request = wasm_request_to_axum_request(&wasm_request).unwrap();

    let axum_response = router.call(request).await.unwrap();

    let response = axum_response_to_wasm_response(axum_response).await.unwrap();
    response
}
```

The service worker overrides fetch so any network requests will go through the Wasm app rather than
out to the internet. This follows a similar pattern seen in the rust example above. I leveraged
the [axum-browser-adapter](https://github.com/logankeenan/axum-browser-adapter) to map the request and response.

```js
self.addEventListener('fetch', event => {
    event.respondWith((async () => {
        const {app, WasmRequest} = wasm_bindgen;
        const wasmRequest = await requestToWasmRequest(event.request, WasmRequest);

        const wasmResponse = await app(wasmRequest);

        return wasmResponseToJsResponse(wasmResponse);
    })());
});
```

The same app can be integrated server-side so the first initial page load will be fast.

```rust
#[tokio::main]
async fn main() {
    let app = create_app();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

This proof of concept of running a Rust server on the client presents an interesting alternative for web development. By
merging the best of both server-side and client-side, it creates a more responsive and efficient user experience.

## Source Code & Demos

The demo app is part of my [rust-everywhere](https://github.com/logankeenan/rust-everywhere) repo. It's collection of
projects experimenting with how to _run_ a server-side Rust app in various platforms.

* app - A basic note-taking application [source](https://github.com/logankeenan/rust-everywhere/tree/main/app)
* server-side ([demo](https://rust-everywhere-server-side.logankeenan.com/)) - Standard server side
  implementation [source](https://github.com/logankeenan/rust-everywhere/tree/main/server-side)
* spa ([demo](https://rust-everywhere-spa.pages.dev/)) - A client-side server leveraging service
  workers [source](https://github.com/logankeenan/rust-everywhere/tree/main/spa)
* spa-server ([demo](https://rust-everywhere-spa-server.logankeenan.com/)) - Server-side implementation for the first
  initial page load followed by the client-side server for
  subsequent _requests_ [source](https://github.com/logankeenan/rust-everywhere/tree/main/spa-server)

