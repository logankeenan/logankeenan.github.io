+++
title = "Past Rust Experiments: Positivelys & Shared Spaces"
description = ""
date = 2023-09-16
+++

While learning Rust, I took the opportunity to create two different projects to explore its capabilities. The first
project, called Positivelys, serves as "your personal feed of positive moments. Make positive thinking a healthy habit
by saving daily positive moments via notes and images." The second project, Shared Spaces, was an experiment in P2P and
WebRTC file sharing between browsers. Both projects were primarily written in Rust.

## Positivelys

The objective of Positivelys was to write the majority of the codebase in Rust and have the iOS and Android applications
interact with it via FFI (Foreign Function Interface). To make the UI uniform across platforms, I opted for a WebView on
both Android and iOS. Unlike typical WebView mobile apps, this one had a unique approach to communication.

In this setup, HTTP requests were sent across the FFI boundary to the Rust application, which would then return HTTP
responses. These responses could range from HTML to JSON. When an HTML response was
received, it was injected directly into the WebView, thereby mimicking a web server's behavior. This server-like
app was included an embedded SQLite database for storing records and direct file system access for managing
images. The server pattern felt familiar and was straightforward to work with, which made the development process much
smoother.

The project encountered some bugs related integrations with the differing native UI navigation
patterns. Around this time, I started a new job and welcomed a new child into the family. These significant life changes
left me with limited time to address the issues, leading to the difficult decision to remove the app from the stores.

[Website](https://web.archive.org/web/20220310074038/https://positivelys.com/)
[Website Source](https://github.com/logankeenan/positivelys-media)
[Play Store](https://web.archive.org/web/20210713201115/https://play.google.com/store/apps/details?id=com.cultivatedsoftware.positivelys)
[App Store](https://web.archive.org/web/20210713201052/https://apps.apple.com/us/app/positivelys/id1498984121)
[App Source](https://github.com/logankeenan/positivelys)

<div>
<div style="width: 50%; box-sizing: border-box; float: left;">
{{ resize_image(path="../static/images/positivelys-android_first_open.png", alt="Garlic on a table with child standing on the stems") }}
</div>
<div style="width: 50%; box-sizing: border-box; float: left;">
{{ resize_image(path="../static/images/positivelys-ios_with_positivelys.png", alt="Garlic on a table with child standing on the stems") }}
</div>
</div>
<div style="clear:both;"></div>

## Shared Spaces

Shared Spaces was a project aimed at learning Rust while experimenting with WebAssembly, P2P, and WebRTC. The project
was bifurcated into two main components: an app compiled to
WebAssembly (WASM) for the client-side and a Rust-based server for account management and establishing WebRTC
connections via WebSockets. While the server application adhered to traditional server-side patterns, albeit in Rust,
the WASM component was more of an experimental with what could be achieved with WebAssembly and Rust.

The client-side component was more interesting. Implemented in Rust and compiled to WebAssembly, it functioned as
a basic HTTP server. HTTP requests would be sent across the JavaScript-WASM boundary, and the Rust app would return HTTP
responses. This setup allowed the majority of the code logic to reside within the Rust application. The JavaScript layer
was kept minimal, primarily responsible for constructing HTTP requests and updating the DOM based on the received HTTP
responses.

After establishing a WebRTC connection between two browser clients, the next challenge was file sharing. Consistent with
the rest of the project, I wanted to stick to standard server-side patterns for this as well. To accomplish this, I
devised a method to send HTTP requests and receive HTTP responses across the WebRTC connection. This request/response
pattern streamlined the process, making it simpler to implement peer-to-peer file sharing. I further facilitated file
transfers by implementing a basic BitTorrent-like protocol, where files were chunked, hashed, and reassembled at the
receiving end.

[Client-side Source](https://github.com/logankeenan/shared-spaces-app-spike) [Server Source](https://github.com/logankeenan/shared-spaces-server-spike)




