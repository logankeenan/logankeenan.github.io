+++
title = "Running a Rust Server in-browser as an SPA"
description = "Running a Rust server compiled to WASM in the browser as a single page application"
date = 2022-02-19
draft = true
+++

I want to use standard server-side web app patterns. No messy build tools, no learning UI frameworks, no transpilers, or
any of the other complexity involved with development today. Just a server, HTTP requests, HTTP responses, HTML, and
JavaScript when needed. This can all be achieved by _running_ a Rust server in the browser in combination with a bit of
JavaScript. This pattern creates a straight forward familiar developer experience and a quality end result for the user.

## High Level Overview

Let's start with the Rust server. It's mostly the same as any other server. It has routes which call functions that
return responses. That's it. Nothing special, just standard server-side patterns.

How do we create requests? Normally, the browser would create an HTTP request any time an anchor tag is clicked or form
is submitted. In this case, event handlers are added to forms and anchors tags in order to construct the request
manually.

Next, the request needs to be _sent_ to the server. This is
where [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) comes in. In short, it's a tool that generates JavaScript
code to interact with Rust code. It handles all the complexity in how a request can be created as a JavaScript class and
passed to the Rust server.

Finally, the browser is update with the Rust server's HTML response.






 




