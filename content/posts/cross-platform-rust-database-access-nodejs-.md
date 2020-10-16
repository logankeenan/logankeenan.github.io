+++
title = "Cross-Platform Rust: Database Access with Node.js Integration"
description = "A Rust library featuring SQLite database access integrated with Node.js"
date = 2020-10-15
+++

This post will cover integrating the rust-core library created in the previous 
[post](posts/cross-platform-rust-database-access/) with Node.js.


Creating bindings between Rust and Node.js is easy with 
[neon-bindings](https://neon-bindings.com/). Install neon-bindings globally 
with NPM.  The [getting-started](https://neon-bindings.com/docs/getting-started) page has more information
on neon's dependencies for any install issues.

```bash
npm install --global neon-cli
```  

Generate a new neon project inside `/cross-platform-rust-database-access` and install the neon dependencies. 

```bash
neon new nodejs
cd nodejs
npm i
``` 

Before the rust-code is integrated, check to make sure neon is working as expected.

```bash
neon build --release
node lib/index.js
# hello world
``` 

Time to integrate the rust-core library.  First, add rust-core to the `native/cargo.toml` file.
```toml
[dependencies]
neon = "0.5.0"
rust-core = { path = "../../rust-core" }
```

Create the binding between Node.js and Rust by editing `native/src/lib.rs` and add the code below.  It creates a function
 named `call_database` accessible in Node.js and accepts the SQLite path as a string to pass along to rust-core.

```rust 
use neon::prelude::*;
use rust_core::database_test;

fn call_database(mut cx: FunctionContext) -> JsResult<JsString> {
    let database_path = cx.argument::<JsString>(0).unwrap().value();

    let result = database_test(database_path);
    Ok(cx.string(result))
}

register_module!(mut cx, {
    cx.export_function("call_database", call_database)
});
```

Lastly, update the `lib/index.js` file to properly call the newly created binding.
```js
var addon = require('../native');

console.log(addon.call_database('./database.sqlite'));
```

Build and execute the Node.js code.
```bash
neon build --release
node lib/index.js
# Person { id: 1, name: "Ada Lovelace" } 

node lib/index.js
# Person { id: 2, name: "Ada Lovelace" }
```

That's it! While this example is pretty trivial, it's easy to imagine how it can be built upon when creating an app. 

[Source code](https://github.com/logankeenan/cross-platform-rust-database-access/tree/main/nodejs) 


 

 
