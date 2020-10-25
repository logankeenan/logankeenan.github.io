+++
title = "Cross-Platform Rust: Database Access with iOS Integration"
description = "A Rust library featuring SQLite database access integrated with iOS"
date = 2020-10-23
+++
This post is part of a series of posts focused on 
[Cross-Platform Rust: Database Access](/posts/cross-platform-rust-database-access/). 
This post will cover integrating the rust-core library with iOS.  You may also be interested in
[Cross-Platform Rust: Database Access with Node.js Integration](/posts/cross-platform-rust-database-access-nodejs) or
[Cross-Platform Rust: Database Access with Android Integration](/posts/cross-platform-rust-database-access-android)

Start by installing the required rust targets for iOS
```bash
rustup target add aarch64-apple-ios
rustup target add x86_64-apple-ios
```

Install the [cargo lipo](https://crates.io/crates/cargo-lipo) to create a universal iOS library and 
[cbindgen](https://crates.io/crates/cbindgen) to create C headers.

```bash
cargo install cargo-lipo
cargo install cbindgen
```

## iOS Bindings

Create a new rust library for the iOS bindings inside  the `/cross-platform-rust-database-access` directory

```bash
cargo new rust-ios --lib
```

Update the `cargo.toml` file and set the crate type to dynamic system library and a static system library. Also, include
 `rust-core` as a dependency.
```toml
[lib]
crate-type = ["staticlib", "cdylib"]

[dependencies]
rust-core = {path= "../rust-core"}
```

Update `src/lib.rs` wth the code below.  The binding expects a parameter for the database path, so it can be provided to
 rust-core 
```rust
use std::os::raw::{c_char};
use std::ffi::{CString, CStr};
use rust_core::database_test;

#[no_mangle]
pub extern fn call_database(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };

    let database_path = c_str.to_str().unwrap().to_string();
    let database_result = database_test(database_path);

    CString::new(database_result).unwrap().into_raw()
}

#[no_mangle]
pub extern fn call_database_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}
```

Next, build the library and create the corresponding header files.

```
cbindgen src/lib.rs -l c > rust-ios.h
cargo lipo --release
```

# iOS App

Open up Xcode to create a new App using the Storyboard interface and name it ios.

Update the ViewController with the following code.  It gets the path to store the SQLite database file, calls the 
database, and prints the result out to the logs.  

```swift
import UIKit

class ViewController: UIViewController {

    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.
        
        let dirPaths = NSSearchPathForDirectoriesInDomains(.documentDirectory, .userDomainMask, true)

        let result = call_database(dirPaths[0] + "/database.sqlite")
        let query_result = String(cString: result!)
        call_database_free(UnsafeMutablePointer(mutating: result))
        print(query_result)
    }
}
```

Copy over the binary library and header files from rust-ios to the ios project.

```bash
# inside the ios project directory
mkdir -p rust-ios/libs
mkdir -p rust-ios/include

cd ../

cp rust-ios/rust-ios.h ios/rust-ios/include
cp rust-ios/target/universal/release/librust_ios.a ios/rust-ios/libs
```

Add `librust_ios.a` under Frameworks, Libraries, and Embedded Content in your iOS project.

{{ resize_image(path="../static/images/rust-ios-add-binary-to-project.png", alt="Xcode application settings for Frameworks, Libraries, and Embedded Content") }}

Set the Header Search Paths to `ios/rust-ios/include` and Library Search Paths and `ios/rust-ios/libs`.

{{ resize_image(path="../static/images/rust-ios-header-library-paths.png", alt="Xcode application settings for Header Search Paths and Library Search Paths") }}

Set the Objective-C Bridging Header to `ios/rust-ios/include/rust-ios.h`.

{{ resize_image(path="../static/images/rust-ios-bridging-header.png", alt="Xcode application settings for Objective-C Bridging Header") }}

Finally, run or debug the app to see the database call result in the logs.

{{ resize_image(path="../static/images/rust-ios-output.png", alt="Xcode log output: Person { id: 1, name: 'Ada Lovelace' } ") }}


Congratulations, an iOS app is now calling Rust to access a SQLite database! Check out the rust-ios 
[source](https://github.com/logankeenan/cross-platform-rust-database-access/tree/main/rust-ios) and ios 
[source](https://github.com/logankeenan/cross-platform-rust-database-access/tree/main/ios).

Lots of credit goes to [Emil Sjölander](https://visly.app/blog/rust-on-ios) for help with the bindings and iOS project setup.

