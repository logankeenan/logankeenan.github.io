+++
title = "Running a Rust Web Server"
description = "Running a Rust Web Server with Actix web and Handlebar templates"
date = 2019-11-29
+++

The first thing I want to do is get a web server running with Rust. I'm going to walk through installing Rust,
creating a new project, using an Actix-Web server, and using Handlebar templates to render HTML.

## Installing and Running Rust
    
First, the [rustup](https://rustup.rs/) toolchain manager needs to be installed. Rustup is to Rust
as rvm/rbenv is to Ruby, nvm to Node, etc... Run the command below and it'll also install Rustup and the latest
stable version of Rust.
    
```shell 
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Afterwards, you should be able to check the installed version of Rust and Cargo.  [Cargo](https://doc.rust-lang.org/cargo/guide/) is the package manager;
think of it like npm, gem, etc...
    
```shell
$ rustc --version
rustc 1.39.0 (4560ea788 2019-11-04)
$ cargo --version
cargo 1.39.0 (1c6ec66d5 2019-09-30)
```
    
## Creating your First Rust Program
        
Create a new directory for your project. In this case, create a directory called
"running-a-rust-web-server". Change into the newly created directory and run `cargo init`. This will create the
basic files
needed for a Rust app. Plenty of resources exist that go into detail about what initially gets
created, so I wont do that here. Next, run the application with `cargo run` and view it's output.
    
```shell
$ mkdir running-a-rust-web-server
$ cd running-a-rust-web-server
$ cargo init
Created binary (application) package
$ cargo run
Hello, world!
```
## Creating a Web Server
    
Rust has a few different web frameworks to choose from which can be viewed
[here](https://www.arewewebyet.org/topics/frameworks/). I've chosen to use [Actix-Web](https://github.com/actix/actix-web).
Open the Cargo.toml file and add Actix-Web. It should look similar to the file below
    
```toml
[package]
name = "running-a-rust-web-server"
version = "0.1.0"
authors = ["Logan Keenan"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
actix-web = "1.0.9"
```    

You can install your projects dependencies with `cargo fetch`. Cargo will also install
dependencies when you build (`cargo build`). Note for javascript devs, dependencies are not download at a project
level. They are downloaded once per version of Rust. Once dependencies have been fetched then the Cargo.lock
file
is updated automatically.
    
    
Actix-Web has made it super easy to get started. Their [home page](https://actix.rs/)
has some great sample code which we'll use too. Copy the code below to your src/main.rs file and start your Rust
web server (`cargo run`). You should be able to visit [http://localhost:8000](http://localhost:8000) and see Hello World.
    
```rust
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();
}
```

## Rendering HTML with Handlebar templates


    
I decided to go with [Handlebars](href="https://crates.io/crates/handlebars) because it's a really simple
templating language. We'll create a layout page which all other pages can use and then an
index page which will be served at the root ([http://localhost:8000](http://localhost:8000)).

* Add `handlebars = "2.0.2"` to you dependencies in the Cargo.toml file
* Create a file called "./running-a-rust-web-server/src/templates/index.hbs"
* Create a file called "./running-a-rust-web-server/src/templates/layout_page.hbs"
* Copy/paste the contents below into the newly created files.
```handlebars
<!--running-a-rust-web-server/src/templates/index.hbs-->

{{#> layout_page}}
{{#*inline "page_content"}}
    <p>
        {{paragraph_text}}
    </p>
{{/inline}}
{{/layout_page}}
```

```handlebars
<!--running-a-rust-web-server/src/templates/layout_page.hbs-->

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Rust Web Server Example</title>
</head>
<body>
        {{> page_content}}
</body>
</html>
```

Next we need to update our src/main.rs file. The root route is going to register the directory containing the
Handlebar templates and then pass data to the index template to render. The src/main.rs file should look like
the code below. Now you should be able to start the server (`cargo run`) and view the rendered html!
    
```rust
use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse};
use handlebars::Handlebars;
use std::collections::BTreeMap;

fn root(_req: HttpRequest) -> HttpResponse {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".hbs", "./src/templates")
        .unwrap();

    let mut data = BTreeMap::new();
    data.insert("paragraph_text", "This page was rendered using Actix-Web as the web server and handlebars as the templating language!");

    let body = handlebars.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(root))
    })
        .bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000")
        .run()
        .unwrap();
}
```

[Source Code](https://github.com/logankeenan/running-a-rust-web-server)