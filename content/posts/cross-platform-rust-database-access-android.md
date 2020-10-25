+++
title = "Cross-Platform Rust: Database Access with Android Integration"
description = "A Rust library featuring SQLite database access integrated with Android"
date = 2020-10-21
+++
This post is part of a series of posts focused on 
[Cross-Platform Rust: Database Access](/posts/cross-platform-rust-database-access/). 
This post will cover integrating the rust-core library with Android.  You may also be interested in
[Cross-Platform Rust: Database Access with Node.js Integration](/posts/cross-platform-rust-database-access-nodejs).

## Toolchains

Start by installing the required rust targets for Android
```bash
rustup target add aarch64-linux-android 
rustup target add armv7-linux-androideabi 
rustup target add i686-linux-android
```

Next, install the [latest](https://developer.android.com/ndk/downloads) Android NDK (Native Development Kit).  The NDK contains prebuilt toolchains for each version
of Android.  However, the standalone installation will need to be installed because libsqlite3 looks for the toolchain
by name without the Android version in it on the $PATH.  A work-around could be made, but it's easy to install the 
standalone toolchains.  Install the NDK toolchains in an easily accessibly location.  This was tested with Python 2.7.5.  

``` bash
mkdir ~/.NDK
~/Downloads/android-ndk-r21d/build/tools/make_standalone_toolchain.py --force --api 21 --arch arm64 --install-dir ~/.NDK/arm64;
~/Downloads/android-ndk-r21d/build/tools/make_standalone_toolchain.py --force --api 21 --arch arm --install-dir ~/.NDK/arm;
~/Downloads/android-ndk-r21d/build/tools/make_standalone_toolchain.py --force --api 21 --arch x86 --install-dir ~/.NDK/x86;
```

Update the $PATH, so libsqlite3 can access the newly installed toolchains
```shell script
export PATH="~/.NDK/arm64/bin:$PATH"
export PATH="~/.NDK/arm/bin:$PATH"
export PATH="~/.NDK/i686/bin:$PATH"
```

Create a cargo config.toml to point to the newly created toolchains at ~/.cargo/config.toml
```toml
[target.aarch64-linux-android]
ar = ".NDK/arm64/bin/aarch64-linux-android-ar"
linker = ".NDK/arm64/bin/aarch64-linux-android-clang"

[target.armv7-linux-androideabi]
ar = ".NDK/arm/bin/arm-linux-androideabi-ar"
linker = ".NDK/arm/bin/arm-linux-androideabi-clang"

[target.i686-linux-android]
ar = ".NDK/x86/bin/i686-linux-android-ar"
linker = ".NDK/x86/bin/i686-linux-android-clang"
``` 

## Android Bindings

Now that the toolchains are taken care of, create a new rust library for the Android bindings inside 
the `/cross-platform-rust-database-access` directory

```bash
cargo new rust-android --lib
```

Update the cargo.toml to include the rust-core library and [jni](https://crates.io/crates/jni) (Java Native Interface) 
for building the Android bindings. Also, change the `crate-type` to a dynamic system library so it can be loaded by 
Android.

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
rust-core = {path= "../rust-core"}

[target.'cfg(target_os="android")'.dependencies]
jni = { version = "0.11.0", default-features = false }
```

Update the `src/lib.rs` with the code below for the Android binding.  The binding expects a parameter for the database 
path, so it can be provided to rust-core. One thing to point out is the function name corresponds to the package
and class where it'll be consumed in the Android app. Later on, a `MainActivity.java` class will be created in the 
`com.example.android` package scope to consume and execute this binding.

```rust
#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use std::ffi::{CString, CStr};
use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jstring};
use rust_core::database_test;

#[no_mangle]
pub unsafe extern fn Java_com_example_android_MainActivity_calldatabase(env: JNIEnv, _: JObject, j_recipient: JString) -> jstring {
    let database_path_c_string = CString::from(
        CStr::from_ptr(
            env.get_string(j_recipient).unwrap().as_ptr()
        )
    );
    let database_path = database_path_c_string.to_str().unwrap().to_string();

    let database_result = database_test(database_path);

    let output = env.new_string(database_result.to_owned()).unwrap();
    output.into_inner()
}
```

Build the project for the Android targets.
```bash
cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release
```

## Android App
Create a new Android project in the `/cross-platform-rust-database-access` directory targeting API 21 and 
with `com.example.android` as the package.

{{ resize_image(path="../static/images/android-rust-new-project.png", alt="Android Studio - Create New Project") }}

Update the MainActivity.kt with the code below.  The code gets a location for the SQLite file to be saved, passes it to 
the binding and then logs out the result of the database call. 

```kotlin
package com.example.android

import android.os.Bundle
import android.util.Log
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        System.loadLibrary("rust_android")
        var databasePath = packageManager.getPackageInfo(packageName, 0).applicationInfo.dataDir
        Log.d("rust", calldatabase("$databasePath/database.sqlite"))
    }

    external fun calldatabase(to: String): String
}
```

Create a directory for the bindings and copy over the compiled rust-android bindings to the Android project. 
```bash
mkdir android/app/src/main/jniLibs
mkdir android/app/src/main/jniLibs/arm64-v8a
mkdir android/app/src/main/jniLibs/armeabi-v7a
mkdir android/app/src/main/jniLibs/x86

cp rust-android/target/aarch64-linux-android/release/librust_android.so  android/app/src/main/jniLibs/arm64-v8a/librust_android.so
cp rust-android/target/armv7-linux-androideabi/release/librust_android.so  android/app/src/main/jniLibs/armeabi-v7a/librust_android.so
cp rust-android/target/i686-linux-android/release/librust_android.so  android/app/src/main/jniLibs/x86/librust_android.so
```

Finally, start or debug the Android application.  The result of the calldatabase function will be outputted to the logs.


{{ resize_image(path="../static/images/android-rust-database-output.png", alt="Android Studio log output: Person { id: 1, name: 'Ada Lovelace' } ") }}

Congratulations, an Android app is now calling Rust to access a SQLite database! A next step might be serializing the 
result into a class to be used with the Android application.  It's not included in this post, but these build steps could be 
scripted out to ease development.  Check out the rust-android 
[source](https://github.com/logankeenan/cross-platform-rust-database-access/tree/main/rust-android) and android 
[source](https://github.com/logankeenan/cross-platform-rust-database-access/tree/main/android).

Credit is due to [Emil Sjölander](https://visly.app/blog/rust-on-android) for help with toolchain setup and bindings.
