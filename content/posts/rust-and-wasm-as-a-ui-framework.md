+++
title = "Running a Rust Server in-browser as an SPA"
description = "Running a Rust server compiled to WASM in the browser as a single page application"
date = 2022-02-19 
+++

I've been working on novel pattern to make applications and wanted to share. This can be applied to apps running on a
server, in the browser, a Cloudflare worker, desktop/Electron, native apps for iOS/Android, and probably more.

I want to use standard server-side web app patterns. No messy build tools, no learning UI frameworks, no transpilers, or
any of the other complexity involved with development today. It's not that those things are bad, but it's more than I
want to think about. I just want to focus on the user problem and minimal technical overhead. So what does this involve?
Just a server, HTTP requests, HTTP responses, HTML, JavaScript when needed, and updating the browser DOM. This can all
be achieved by _
running_ a Rust server in the browser in combination with a bit of JavaScript. It takes old server-side patterns
enhanced by new technology.

## High Level Overview

Let's start with the Rust server. It's mostly the same as any other server. It has routes which call functions that
return responses. That's it. Nothing special, just standard server-side patterns. We'll
use [tide](https://github.com/http-rs/tide) for the server framework.

How do we create requests and send them to our server? Normally, the browser would create an HTTP request any time an
anchor tag is clicked or form is submitted. In this case, we'll add event handlers to forms and anchors tags in order to
construct the request manually with a bit of JavaScript.

Next, the request needs to be _sent_ to the server. This is
where [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) comes in. In short, it's a tool that generates JavaScript
code to interact with Rust code. It handles all the complexity in how a request can be created as a JavaScript class and
passed to the Rust server as struct.

The Rust server will return an HTML response where the browser will update
leveraging [Morphdom](https://github.com/patrick-steele-idem/morphdom).

There's one last bit of complexity I'll add to this example. We're going to split our code into two parts. One being the
Rust server as a standard Rust lib. The other is the SPA application which will consume the Rust server and maintain the
glue code with the browser. Why do this? By making the rust server a library it allows us to use this pattern and
consume it on many platforms like Cloudflare workers, a typical server, iOS, Android, Electron, and many more.

## The APP

We're going to build a simple note-taking app. It'll perform CRUD operations against a note which has a title and
content. For fun, the content will support markdown. It'll call an API to save our notes to a database. We'll walk
through fetch a list of notes and rendering them. You can check out the source code to view the other CRUD operations
and other details I skip over.

## The Server

This is going to be where all of our app logic lives. It'll leverage my
tide [branch](https://github.com/logankeenan/tide/tree/wasm) as the sever
framework, [surf](https://github.com/http-rs/surf) for making http requests, [askama](https://github.com/djc/askama/)
for templating, [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) for markdown, and of course serde. Let's
call this notes-demo.

```shell
cargo new notes-demo --lib
```

Add the dependencies to Cargo.toml

```toml

[dependencies]
serde = "1.0.132"
askama = "0.11.0"
pulldown-cmark = "0.9.0"
tide = { git = "https://github.com/logankeenan/tide.git", features = ["wasm"], branch = "wasm", default-features = false }
surf = { version = "2.3.2", default-features = false, features = ["wasm-client"] }
```

Next, lets make a public function which will create our Tide server, add a route for listing the notes, and return the
server. Later, the SPA will pass requests to it.

```rust 
pub async fn index(_req: Request<()>) -> tide::Result {
    let notes: Vec<Note> = surf::get("http://localhost:3000/notes")).recv_json().await?;
    let notes_template_model = notes::Index {
        notes: &notes
    };
    let body = notes_template_model.render()?;
    let response = Response::builder(200)
        .body(body)
        .content_type(mime::HTML)
        .build();

    Ok(response)
}

pub fn create() -> Server<()> {
    let mut app = tide::new();
    app.at("/notes").get(index);
    app
}
```

This is a paired down version of the actual app to show the basic setup of a Tide App. The actual app includes basic
CRUD operations against a Note model using HTML forms. Check out repository if you're interested in server
implementation.

## The Rust Parts of the SPA

We've got this server, so now we need to send requests to it. First, lets create another repository where the SPA code
will live. We could have easily put all this in a single repository and not made two. However, I wanted to create a
clear boundary between the app code and the glue code which is the code that specific to the platform, in this case the
browser. In the future, I'll have various implementations of glue code for different platforms.

```bash
cargo new notes-demo-spa --lib
```

Update the cargo.toml file ```[lib]``` and dependencies. We've included the javascript-adapter and the tide-adapter
which will be discussed later.

```toml
wasm-bindgen = "0.2.79"
futures = "0.3.19"
wasm-bindgen-futures = "0.4.29"
tide = { git = "https://github.com/logankeenan/tide.git", branch = "wasm", features = ["wasm"], default-features = false }
notes-demo = { path = "../notes-demo" }
surf = { version = "2.3.2", default-features = false, features = ["wasm-client"] }
javascript-adapter = { git = "https://github.com/rora-rs/javascript-adapter.git", branch = "main" }
tide-adapter = { git = "https://github.com/rora-rs/tide-adapter.git", branch = "main" }
```

Let's create a public function called app. It'll be responsible for receiving a request and sending a response. Thanks
to wasm-bindgen, all we need to do is add a few macros to our structs and functions make them available in Javascript.
It'll generate the javascript code which allows us to interact with Rust compiled to Web Assembly through a Javascript
API. It's awesome. Checkout the wasm-bindgen [docs](https://rustwasm.github.io/wasm-bindgen/) for more information.

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





 




