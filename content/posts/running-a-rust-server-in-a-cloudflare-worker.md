+++
title = "A Rust server-side app in a Cloudflare Worker"
description = "Create a Rust server-side app using the Tide framework, compile it to WASM, and run it in a Cloudflare Worker."
date = 2022-05-07T00:00:01Z
+++

_Rust is extraordinarily portable. This post is the first of many describing how a Rust server-side app can be integrated with
multiple platforms._

## Overview

* Create a Rust library which creates a [Tide](https://github.com/http-rs/tide) server-side app.
* Create a Cloudflare worker with [worker-rs](https://github.com/cloudflare/workers-rs) and integrate the app.

So how does this work? When a Cloudflare worker receives a [request](https://docs.rs/worker/0.0.9/worker/struct.Request.html) it will:

1. Convert the Cloudflare request into a Tide [request](https://docs.rs/tide/latest/tide/struct.Request.html)
2. Create the server-side app and pass the Tide request via
   the [respond](https://docs.rs/tide/latest/tide/struct.Server.html#method.respond) function
3. Convert Tide [response](https://docs.rs/tide/latest/tide/struct.Response.html) to a
   Cloudflare [response](https://docs.rs/worker/0.0.9/worker/struct.Response.html) and return the response from the
   worker.

Note, you don't need to create a separate library for the app, but I'll be referencing the next section in future blog
posts where I'll integrate the same app into various platforms.

## Creating the App

We're going to create a small portion of a note taking app. We'll create the index route which will render a list of
notes. The app uses an [API](https://github.com/rora-rs/notes-demo-api) to persists notes, makes calls to the API
with [surf](https://github.com/http-rs/surf), and uses [askama](https://github.com/djc/askama/) for templating. Feel
free to view the complete [source code](https://github.com/rora-rs/notes-demo) or try out
the [demo](https://notes-demo-cf-worker.logankeenan.workers.dev/).

Create a repo for the app.

```shell
cargo new --lib notes-demo
```

Update the Cargo.toml. For now, we need to use my fork of Tide to enable WASM. I have an
open [PR](https://github.com/http-rs/tide/pull/877).

```toml
edition = "2021"

[dependencies]
serde = "1.0.132"
askama = "0.11.0"

[target.'cfg(target_arch = "wasm32")'.dependencies]
tide = { git = "https://github.com/logankeenan/tide.git", features = ["wasm"], branch = "wasm", default-features = false }
surf = { version = "2.3.2", default-features = false, features = ["wasm-client"] }

# This is needed for other non-wasm platforms
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
tide = { git = "https://github.com/logankeenan/tide.git", branch = "wasm" }
surf = { version = "2.3.2" }
```

Update the lib.rs to include a model that represents a note.

```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Note {
    pub id: i32,
    pub title: String,
    // the note content can be rendered to markdown. See the source for more details
    pub markdown: String,
    pub update_at: Option<String>,
    pub created_at: String,
}
```

Create a file to render the markup in `notes-demo/template/notes/index.html`

```html
<!doctype html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
</head>
<nav>
    <a href="/notes">Notes</a>
    <a href="/notes/new">New Note</a>
</nav>

<h1>Notes</h1>
<ul>
    {% for note in notes %}
    <li>
        <a href="/notes/{{note.id}}">{{note.title}}</a>
    </li>
    {% endfor %}
</ul>

</body>
</html>
```

Create a struct to represent the model _bound_ to the template.

```rust
// src/lib.rs

use askama::Template;

#[derive(Template)]
#[template(path = "notes/index.html")]
pub struct IndexTemplate<'a> {
    pub notes: &'a Vec<Note>,
}
```

Now, lets create a route which will call the API for notes and render the result as HTML.

```rust
// src/lib.rs

use surf::Response as SurfResponse;
use tide::{Request, Response, Server};

pub async fn index(_: Request<()>) -> tide::Result {
    let mut api_response: SurfResponse = surf::get(
        "https://rora-notes-demo-api.herokuapp.com/notes"
    ).await?;

    let notes: Vec<Note> = api_response.body_json().await?;
    let notes_template_model = IndexTemplate {
        notes: &notes
    };
    let body = notes_template_model.render()?;

    let response = Response::builder(200)
        .body(body)
        .build();
    Ok(response)
}
```

Create a public function which will create the Tide server with the index route and return it.

```rust
// src/lib.rs

pub fn create() -> Server<()> {
    let mut app = tide::new();
    app.at("/").get(index);

    app
}
```

Finally, build the library.

```shell
cargo build --target wasm32-unknown-unknown
```

## Integrating the App into a Cloudflare Worker

Next, we need to create a Cloudflare worker which forwards requests to our app and return the responses.

Install [@cloudflare/wranger](https://github.com/cloudflare/wrangler) and create a new rust project.

```shell
npm i @cloudflare/wrangler -g
wrangler generate notes-demo-cf-worker --type rust
```

Update the Cargo.toml file with the local notes-demo app from, my Tide fork, and the edition.

```toml
edition = "2021"

log = "0.4.17" # forcing a version of log due to conflicts
notes-demo = { path = "../notes-demo" }
tide = { git = "https://github.com/logankeenan/tide.git", features = ["wasm"], branch = "wasm", default-features = false }
```

Now lets update src/lib.rs to convert the Cloudflare worker request to Tide request.

```rust
// src/lib.rs
use std::str::FromStr;
use tide::http::{Method, Url, Request as TideRequest, Response as TideResponse};

let method = Method::from_str(req.method().to_string().as_str()).unwrap();
let url = Url::parse(req.url().unwrap().as_str()).unwrap();
let tide_request = TideRequest::new(method, url);
// add headers and body too
```

Create the app server and pass the Tide request to it

```rust
// src/lib.rs

let app_server = notes_demo::create();
let mut tide_response: TideResponse = app_server.respond(tide_request).await.unwrap();
```

Convert the Tide response to a Cloudflare response and return the result.

```rust
// src/lib.rs

let response = Response::from_html(tide_response.body_string().await.unwrap()).unwrap();
let status_code: u16 = tide_response.status().to_string().parse().unwrap();
let response = response.with_status(status_code);
// add headers too

return Ok(response);
```

Run the Cloudflare worker with `wrangler dev`. You should see some HTML with a Notes heading and a few links above it.
This isn't the complete app, but you can try the [demo](https://notes-demo-cf-worker.logankeenan.workers.dev/) and
view [source](https://github.com/rora-rs/notes-demo) for the complete app.

## What's Next?

It'd be pretty easy to create a much more complex app with all the perks of Rust and it's ecosystem. Since this just
compiles to WASM, we could also run this app in the browser. I'll have another post on that. Integrating Cloudflare's
recently [announced](https://blog.cloudflare.com/introducing-d1/) D1 SQL database would allow for a complete full-stack
app. 
