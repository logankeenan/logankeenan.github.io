+++
title = "A Rust Server App Compiled to WASM as an SPA"
description = "Compiling a standard server-side Rust app to WASM and running it in-browser as an SPA"
date = 2022-06-03
+++

Rust is extraordinarily portable. This blog post is going to go over integrating a traditional server-side Rust app with
the browser, so the server-side app can be run as a SPA.

Checkout the [demo](https://notes-demo-spa.pages.dev/) and/or [source](https://github.com/rora-rs/notes-demo-spa).

## Overview

First, let's take a step back and talk about traditional server-side application patterns. What does a web app look like
in the absence of JavaScript? Everything relies on HTTP messages. Click on a link - the browser makes an HTTP request
and renders the HTML response. Submit a form - the browser creates an HTTP message, serializes the form as the HTTP
message body, and often follows the location header of the
response ([POST, Redirect, GET](https://en.wikipedia.org/wiki/Post/Redirect/Get)). Basically, the browser makes an HTTP
request anytime we want the page to be updated.

The server-side Rust app can be compiled to WASM, so it can receive HTTP requests and return HTTP responses all inside
the browser. We just need a way to make our browser send HTTP requests to it and update the page with the HTTP response.
We can do this with a small amount of JavaScript. Anytime a user clicks on an anchor tag then we'll create an HTTP GET
request where the URL is the anchor tag href, and send it to the Rust app. Anytime a user submits a form then we'll
create an HTTP request where the HTTP verb is the form method, the HTTP URL is the form action, and the HTTP body is the
serializer form data.

Next, the response needs to update the browser. For the sake of a short blog post, we can set
the `document.documentElement.innerHTML`
to the HTTP response body. A better solution would be to use [morphdom](https://github.com/patrick-steele-idem/morphdom)
to update the page.

## Creating the App

We're going to create a note taking app. I went over this in a previous blog post. So go checkout the
[Creating the App](/posts/running-a-rust-server-in-a-cloudflare-worker/#creating-the-app) section of
[_A Rust App in a Cloudflare Worker_](/posts/running-a-rust-server-in-a-cloudflare-worker) and then come back.

## Integrating the App into the Browser

Let's start by creating a new Rust library. With it, we'll integrate the notes-demo app, compile it to WASM, create a
JavaScript bridge to make requests, and update the browser with the response.

```bash
cargo new notes-demo-spa --lib
```

Update the cargo.toml file. We've included `rora-javascript-adapter` which is a library I created to define a JsRequest
and JsResponse. We can use them as structs in Rust or classes in JavaScript. They allow interoperability of our HTTP
messages between the browser and our Rust App. The `rora-tide-adapter` is a library that helps convert our JsRequests to
TideRequests and Tide responses to JsResponses. Checkout the source code for
[rora-javascript-adapter](https://github.com/rora-rs/javascript-adapter) and
[rora-tide-adapter](https://github.com/rora-rs/tide-adapter) to learn more.

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2.79"
futures = "0.3.19"
wasm-bindgen-futures = "0.4.29"
tide = { git = "https://github.com/logankeenan/tide.git", branch = "wasm", features = ["wasm"], default-features = false }
notes-demo = { path = "../notes-demo" }
surf = { version = "2.3.2", default-features = false, features = ["wasm-client"] }
rora-javascript-adapter = "0.0.2"
rora-tide-adapter = { git = "https://github.com/rora-rs/tide-adapter.git", branch = "main" }
```

Let's create a public function called app. It'll be responsible for receiving a request and sending a response. Thanks
to wasm-bindgen, all we need to do is add the `#[wasm_bindgen]` macro to our app function to make it available in
Javascript. It'll generate the javascript code which allows us to interact with Rust compiled to WASM through a
Javascript API. It's awesome. Checkout the wasm-bindgen [docs](https://rustwasm.github.io/wasm-bindgen/) for more
information. Notice

The body of our app function does it a few things. It creates the notes_demo app, converts the JsRequest to a Tide
request, passes the Tide request to the app via
the [respond](https://docs.rs/tide/latest/tide/struct.Server.html#method.respond)
function, converts the Tide response to a JsResponse, and returns the JsResponse.

```rust
use tide::http::{Request as TideRequest, Response as TideResponse};
use wasm_bindgen::prelude::*;
use tide::{Body, Middleware, Next, Request, Response};

// We're also making the JsRequest and JsResponse public so we can use them in the browser 
pub use rora_javascript_adapter::{JsRequest, JsResponse};

#[wasm_bindgen]
pub async fn app(js_request: JsRequest) -> JsResponse {
    let mut app = notes_demo::create();

    let tide_request: TideRequest = rora_tide_adapter::javascript::to_tide_request(js_request);
    let tide_response: TideResponse = app.respond(tide_request).await.unwrap();

    rora_tide_adapter::javascript::to_response(tide_response).await
}
```

Now we can compile our Rust code to WASM, so it'll run in the browser. First, we need to install the WASM target for
Rust.

```shell
rustup target add wasm32-unknown-unknown
```

Next, install wasm-bindgen

```shell
cargo install -f wasm-bindgen-cli
```

Finally, lets compile to WASM and create the JavaScript bindings.

```shell
cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/notes_demo_spa.wasm --out-dir ./dist/wasm --target web
```

## The JavaScript Adapter

The JavaScript will be responsible for hijacking anchor tag clicks and form submissions. We'll create our own HTTP
request using JsRequest, pass it to the server, and then update the document with our response body. There are other
edge cases we need to account for like client-side routing or non-200 response codes, but that's beyond the scope of
this post.

Let's start by creating a new index.html page at the root of our project. We'll start by calling init with the output of
wasm-bindgen. This is all just boilerplate code for wasm-bindgen to prepare our app before we can start invoking it.
Create a JsRequest using the current pages url, pass it to our app, and update the browser page with the response.

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
</head>
<body>
<script type="module">
    import init, {app, JsRequest} from '../dist/wasm/notes_demo_spa.js'

    (async () => {
        const url = new URL('/dist/wasm/notes_demo_spa_bg.wasm', window.location.href);
        await init(url);

        const jsRequest = new JsRequest(window.location.href, "GET");
        const response = await app(jsRequest);

        document.documentElement.innerHTML = response.body;
    })();
</script>
</body>
</html>
```

Now that we have our code in place, we can run any arbitrary HTTP server to serve our HTML, JS, and WASM files. I like
to use [basic-http-server](https://github.com/brson/basic-http-server). The page should render with a Notes heading and
a few links. We only did part of the app, but you can check out the [source](https://github.com/rora-rs/notes-demo-spa)
to learn more.

Our example code isn't that functional. It just renders one page. We'd need to add some additional JavaScript to handle
when anchor tags are clicked, forms are submitted, some client-side routing, and probably a few other edges cases.
However, the majority of the app could be written in Rust and integrated with the browser with a fairly small amount of
JavaScript.

## Additional Thoughts

This pattern also allows for server-side rendering via a regular server or a Cloudflare worker and then running the app
client-side after the initial page load.
Both the [TTI](https://developer.mozilla.org/en-US/docs/Glossary/Time_to_interactive)
and [LCP](https://developer.mozilla.org/en-US/docs/Web/API/LargestContentfulPaint) would be very low. The app doesn't
need to do any [rehydration](https://en.wikipedia.org/wiki/Hydration_(web_development)) or execute any JS code before
the user can interact with it. The client simply interacts with the server if the client-side code hasn't finished
processing.