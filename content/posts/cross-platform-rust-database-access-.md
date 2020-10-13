+++
title = "Cross-Platform Rust: Database Access"
description = "A series of blogs posts demonstrating cross-platform database access via Rust"
date = 2020-10-13
+++

This is a series of blogs posts which will demonstrate SQLite database access written in Rust and cross-compiled
to run on iOS, Android and Node.js. Why do this?  Most apps need some sort of way to store data on the device.  Each of
these platforms have their own way to store data, so storing data is implemented three different ways.  Duplicate
code for the same feature isn't ideal.  Creating the database access layer in Rust and compiling for each platform 
significantly reduces the amount of coding needed for cross-platform apps.

## Using SQLite with Rust

This first post will cover creating the core rust code that accesses the database and demonstrates creating records
and returning a query result.  The code is pretty basic, but demonstrates the building blocks needed to build out an 
app.

Let's start by creating a directory which will eventually contain the code for the different platforms.

```bash 
$ mkdir cross-platform-rust-database-access
$ cd cross-platform-rust-database-access
```

Next, create a Rust library which will contain our database access code and be cross-compiled.
```bash
$ cargo new rust-core --lib
```

Update the cargo.toml file to include [rusqlite](https://crates.io/crates/rusqlite) as a dependency.  Rusqlite uses the
C implementation of SQLite which is built from source by leveraging the "bundled" feature.  
```toml
[dependencies]
rusqlite = {version= "0.24.1", features=["bundled"]}
```

Finally, let's get to the code.  Feel free to copy/paste the code below.  Most of this is pretty standard code, 
except the database_location is used as a parameter to the function. Why?  Each platform has different requirements 
around file access, so the platform needs to be provide where the Rust code can read/write to the SQLite file.  

```rust
use rusqlite::{params, Connection};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
}

pub fn database_test(database_location: String) -> String {
    let connection = Connection::open(database_location).unwrap();

    connection.execute(
        "CREATE TABLE IF NOT EXISTS person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL
                  )",
        params![],
    ).unwrap();

    connection.execute(
        "INSERT INTO person (name) VALUES (?1)",
        params!["Logan Keenan"],
    ).unwrap();

    let mut stmt = connection.prepare("SELECT id, name FROM person").unwrap();
    let person_iter = stmt.query_map(params![], |row| {
        Ok(Person {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
        })
    }).unwrap();

    let option = person_iter.last().unwrap().unwrap();

    format!("{:?}", option)
}
```

Build the project to make sure everything compiles.

```bash 
$ cargo build
```

The next post will cover how to integrate the newly created rust-core library in Node.js.
