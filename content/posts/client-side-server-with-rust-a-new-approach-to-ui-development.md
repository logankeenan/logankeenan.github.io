+++
title = "Client-Side Server with Rust: A New Approach to UI Development"
description = "Explore how to build a client-side server using Rust and WebAssembly, offering a unique UI development alternative. This guide demonstrates compiling Rust to run in the browser, enhancing performance and user experience. Ideal for developers interested in innovative web solutions, Rust programming, and WebAssembly."
date = 2023-08-02
+++

This blog post demonstrates how to leverage the simplicity of server-side patterns while enjoying the benefits of
client-side applications using a Rust server on the client.

## The Current State

In server-side applications, HTML is delivered upon a page request. User interactions, such as form submissions or link
clicks, prompt HTTP requests to the app, which then responds with HTML. This methodology offers a fast initial page load
and arguably more straightforward development. However, continuous server round-trips can degrade the user experience
due to latency.

On the other hand, client-side applications improve user experience with snappier interactions due to dynamic updates
within the browser. However, drawbacks include a potentially slow initial page load, performance implications from
executing a large amount of JavaScript, and a higher overall complexity.

## A Server on the Client?

The idea of running a server on the client might sound unusual. In essence, it means compiling a Rust server into
WebAssembly, which can then be called directly in the browser. My
Previous [solution](https://logankeenan.com/posts/a-rust-server-app-compiled-to-wasm-as-an-spa/) had its
drawbacks, including event hijacking, DOM updates, JS integration, and performance concerns. Then, I discovered Richard
Anaya's brilliant Wasm Service Worker [POC](https://github.com/richardanaya/wasm-service).

Service workers, often used in offline web apps, intercept HTTP requests and cache responses. Subsequent requests access
the cached response instead of making new HTTP requests. In our case, we call our Wasm app with the request and return
the response, so the browser thinks it's communicating with a remote server.

This approach brings several advantages:

- Enhances user experience with client-side code execution.
- Boosts performance since operations run on a separate thread.
- Simplifies development by only requiring management of the server app.

We also experience faster initial page loads since the same Rust app server code can render the initial page on the
server. This allows the user to interact with the page immediately, even before the client-side app has loaded,
defaulting to server-side rendering if necessary.

## How is this any different?

Client-side applications often require developers to handle a multitude of considerations:

* Routing and managing the History API for navigating between pages
* Updating the Document Object Model (DOM) to reflect changes in the user interface
* Managing application state across different page navigations
* Recreating native browser features with JavaScript, such as maintaining the scroll position when navigating back to a
  previous page
* Adding or removing JavaScript event listeners and/or libraries as the user moves between pages

In contrast, client-side server applications free developers from these concerns, as these aspects are handled directly
by the browser itself. This allows for a more efficient and streamlined development process.

## How to Make It Interactive

Anchor tags and forms can provide interactivity, but for more dynamic interactions, you have options. Integrate htmx,
load your favorite JS framework, or go wild with vanilla JS. The browser, thinking it communicates with a server and
rendering
HTML, imposes no limitations.

## The Proof of Concept

I will demonstrate this idea using the [Axum](https://github.com/tokio-rs/axum) framework and Rust. However, any server
framework that compiles to Wasm and
is callable with an HTTP request could achieve this. I created a basic note-taking app that allows for creation,
reading, updating, and searching. You can check out the [Source Code & Demos](#source-code-demos).

I also created a Rust library that exposes a function called `create_app`. This function sets up the Axum router with
routes and encapsulates the entire app, allowing integration across various platforms.

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

Browser integration is done through a Rust library that compiles to Wasm and integrate with JavaScript
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

The same app can be integrated server-side to ensure a fast first page load.

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

