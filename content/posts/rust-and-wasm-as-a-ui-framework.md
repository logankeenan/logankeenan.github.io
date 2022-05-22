+++
title = "Using a Server-Side Rust App as an SPA"
description = "Compiling a standard server-side Rust app to WASM and running it in-browser as an SPA"
date = 2022-02-19 
+++

Rust is extraordinarily portable. This blog post is going to go over integrating a traditional server-side Rust app with
the browser, so the server-side app can be run as a SPA. 

## Overview

First, let's take a step back and talk about the traditional server-side application patterns.  What does a web app 
look like in the absence of JavaScript?  Everything relies on HTTP messages. Click on a link - the browser makes a 
HTTP request and renders the HTML response. Submit a form to create/edit/delete data - the browser creates a HTTP, 
serializes the form as the HTTP message body, and often follows the location header of the response ([POST, Redirect, GET](https://en.wikipedia.org/wiki/Post/Redirect/Get)).
Basically, the browser makes a HTTP request anytime we want the page to be updated.

We have a server-side Rust app. It can receive HTTP requests and returns HTTP response. It can be compiled to WASM so 
the whole app can be run in browser.  We just need a way to make our browser send HTTP requests to it and update the 
page with the HTTP response. We can do this with a small amount of JavaScript. Anytime a user clicks on an anchor tag then
we'll create a HTTP GET request where the URL is the anchor tag href, and send it to the Rust app. Anytime a user submits
a form then we'll create an HTTP request where the HTTP verb is form method, the HTTP URL is the form action is. The
HTTP body is the encoded form data.  

Next, the response needs to update the browser.  For the sake of a short blog post, we can set the `document.documentElement.innerHTML` 
to the HTTP response body.  A better solution would be to use [morphdom](https://github.com/patrick-steele-idem/morphdom)
to update the page.
 
## Creating the App
We're going to create a note taking app.  I went over this in a previous blog post. So go checkout the 
[Creating the App](/posts/running-a-rust-server-in-a-cloudflare-worker/#creating-the-app) section of 
[_A Rust App in a Cloudflare Worker_](/posts/running-a-rust-server-in-a-cloudflare-worker) and then come back.

## Integrating the App into the Browser

Let's start by creating a new Rust library. With it, we'll integrate the notes-demo app, compile it to WASM,
and create a JavaScript bridge to make requests, and update the browser for respones.

```bash
cargo new notes-demo-spa --lib
```

Update the cargo.toml file. We've included `rora-javascript-adapter` which is a library I created to define a 
JsRequest and JsResponse.  We can use them as structs in Rust or classes in JavaScript.  They allow interoperability of 
our HTTP messages between the browser and our Rust App.  The `rora-tide-adapter` is a library that helps convert our 
JsRequests to TideRequests and Tide responses to JsResponses.  Checkout the source code for 
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
to wasm-bindgen, all we need to do is add a few macros to our structs and functions make them available in Javascript.
It'll generate the javascript code which allows us to interact with Rust compiled to Web Assembly through a Javascript
API. It's awesome. Checkout the wasm-bindgen [docs](https://rustwasm.github.io/wasm-bindgen/) for more information.  Notice

```rust
use tide::http::{Request as TideRequest, Response as TideResponse};
use wasm_bindgen::prelude::*;
use tide::{Body, Middleware, Next, Request, Response};

pub use rora_javascript_adapter::{JsRequest, JsResponse};

#[wasm_bindgen]
pub async fn app(js_request: JsRequest) -> JsResponse {
    let mut app = notes_demo::create();

    let tide_request: TideRequest = tide_adapter::javascript::to_tide_request(js_request);
    let tide_response: TideResponse = app.respond(tide_request).await.unwrap();

    tide_adapter::javascript::to_response(tide_response).await
}
```

Let's unpack this because there's quite a bit going on. The app function takes a JsRequest as a parameter and returns a
JsResponse. JsRequest and JsResponse come
from [rora-javascript-adapter](https://docs.rs/rora-javascript-adapter/latest/rora_javascript_adapter/) which is a crate
I created to make it easy to send HTTP messages from JavaScript to WASM/Rust and vice versus. Next, we convert the
JsRequest to a TideRequest. Again, this is handled by crate
called [rora-tide-adapter](https://github.com/rora-rs/tide-adapter). The tide_request is passed to the app resulting in
a tide_response. The tide_response is converted to a JsResponse and returned from the function.

We have our rust code, now we need to compile it to WASM and leverage wasm-bindgen to create the bindings between 
JavaScript and WASM. I assume Rust is already installed, so we just need to install the WASM compile target

```shell
rustup target add wasm32-unknown-unknown
```

Install wasm-bindgen
```shell
cargo install -f wasm-bindgen-cli
```

Finally, lets compile to WASM and create the JavaScript bindings. We'll put the output in `/dist/wasm`.  Note, a release 
build could be done by appending `--release` to the first command and pointing at the release target rather than debug 
for wasm-bindgen.
```shell
cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/notes_demo_spa.wasm --out-dir ./dist/wasm --target web
```

## The JavaScript

Now, we have a server that receives and responds to request.  It's compiled to WASM and can be called from JavaScript.  
How can we send requests to our server?  How would a normal server rendered web app work? Clicking on anchor tags make
GET requests to the href. Submitting forms make POST requests to the action with the form inputs serialized as the body.
That's about it.  To call our server we just need to addEventListeners to those user actions, create our requests, and send
them to our server. 

What about the response? 





 




