+++
title = "Running a Rust Server in a Cloudflare Worker"
description = "Create a Rust server using the tide framework, compile it to WASM, and run it in a Cloudflare Worker."
date = 2022-05-07T00:00:01Z
+++

_Rust is extraordinarily portable. This post is the first of many describing how an app can be written in Rust as a
server-side app and integrated into multiple platforms._

## Overview

1. Create a Rust library which creates app leveraging the [tide](https://github.com/http-rs/tide) server framework.
2. Create a Cloudflare worker with [worker-rs](https://github.com/cloudflare/workers-rs).
3. Integrate the app with the Cloudflare worker.

So how does this work? When the Cloudflare worker receives
a [request](https://docs.rs/worker/0.0.9/worker/struct.Request.html) then convert it to a
tide [request](https://docs.rs/tide/latest/tide/struct.Request.html), pass the request to the
app's [respond](https://docs.rs/tide/latest/tide/struct.Server.html#method.respond) function, convert the
tide [response](https://docs.rs/tide/latest/tide/struct.Response.html) to a
Cloudflare [response](https://docs.rs/worker/0.0.9/worker/struct.Response.html), and return it from the worker.

## Creating the App

We're going to create a small portion of a note taking app.The app uses
an [API](https://github.com/rora-rs/notes-demo-api) to persists notes, makes calls to the API
with [surf](https://github.com/http-rs/surf), and uses [askama](https://github.com/djc/askama/) for templating. Feel
free to view the complete [source code](https://github.com/rora-rs/notes-demo) or try out
the [demo](https://notes-demo-cf-worker.logankeenan.workers.dev/). Alright, lets get started.

Create a repo for the app.

```shell
cargo new --lib notes-demo
```

Update the dependencies in Cargo.toml. For now, we need to use my fork of tide which enables wasm. I have an
open [PR](https://github.com/http-rs/tide/pull/877).

```toml
[dependencies]
serde = "1.0.132"
askama = "0.11.0"

[target.'cfg(target_arch = "wasm32")'.dependencies]
tide = { git = "https://github.com/logankeenan/tide.git", features = ["wasm"], branch = "wasm", default-features = false }
surf = { version = "2.3.2", default-features = false, features = ["wasm-client"] }
```



## Questions

* **Can I use this today?**

  Yes and no, I currently have a [PR](https://github.com/http-rs/tide/pull/877) to allow WASM support in tide. Feel free
  to voice your support. For now, my tide [fork](https://github.com/logankeenan/tide) will work. 