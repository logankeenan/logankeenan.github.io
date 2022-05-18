+++
title = "Running a Rust Server in a Cloudflare Worker"
description = "Create a Rust server using the tide framework, compile it to WASM, and run it in a Cloudflare Worker."
date = 2022-05-07T00:00:01Z 
+++

_Rust is extraordinarily portable. This post is the first of many describing how a server-side app can be written in
Rust and integrated with multiple platforms._

## Overview

1. Create a Rust library which encapsulates the app as a [tide](https://github.com/http-rs/tide) server.
2. Create a Cloudflare worker with [worker-rs](https://github.com/cloudflare/workers-rs) and integrate the app.

So how does this work? When the Cloudflare worker receives
a [request](https://docs.rs/worker/0.0.9/worker/struct.Request.html) it converts it to a
tide [request](https://docs.rs/tide/latest/tide/struct.Request.html), creates the app server, passes the tide request to the
app's [respond](https://docs.rs/tide/latest/tide/struct.Server.html#method.respond) function, convert the
tide [response](https://docs.rs/tide/latest/tide/struct.Response.html) to a
Cloudflare [response](https://docs.rs/worker/0.0.9/worker/struct.Response.html), and returns the response from the worker.

You don't need to create a separate library for the app sever code, but I'll be referencing the next section in future 
blog posts where I'll integrate the same app in different platforms. 

## Creating the App

We're going to create a small portion of a note taking app, rendering a list of notes.  The app uses
an [API](https://github.com/rora-rs/notes-demo-api) to persists notes, makes calls to the API
with [surf](https://github.com/http-rs/surf), and uses [askama](https://github.com/djc/askama/) for templating. Feel
free to view the complete [source code](https://github.com/rora-rs/notes-demo) or try out
the [demo](https://notes-demo-cf-worker.logankeenan.workers.dev/).

Create a repo for the app.

```shell
cargo new --lib notes-demo
```

Update the dependencies in Cargo.toml. For now, we need to use my fork of tide to enable WASM. I have an
open [PR](https://github.com/http-rs/tide/pull/877).

```toml
[dependencies]
serde = "1.0.132"
askama = "0.11.0"

[target.'cfg(target_arch = "wasm32")'.dependencies]
tide = { git = "https://github.com/logankeenan/tide.git", features = ["wasm"], branch = "wasm", default-features = false }
surf = { version = "2.3.2", default-features = false, features = ["wasm-client"] }

# This is needed for other non-wasm platforms
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
tide = {git = "https://github.com/logankeenan/tide.git", branch = "wasm" }
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

Create the struct to represent the model _bound_ to the template.

```rust
// src/lib.rs

use askama::Template;
#[derive(Template)]
#[template(path = "notes/index.html")]
pub struct IndexTemplate<'a> {
    pub notes: &'a Vec<Note>,
}
```


Now, lets create a route which will call the API for notes and render them to HTML.

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

Create a public function for our library to expose which will create our tide server with the index route 
and return it.

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

