+++

title = "Hybrid Apps with Rust"
description = "Leveraging Rust to create a local server for build hybrid apps"
date = 2021-02-25

+++

I'm working on an app which I'll hopefully release soon. It's a hybrid app. No, it's not a WebView that loads remote
content from a server. It's not a WebView that loads a JavaScript app. Rust isn't compiled to WASM and running in a
WebView. It's a *local* server built with Rust that executes on iOS and Android. The native code makes a request to
the *local* server, receives a response, and renders the HTML response in a WebView. It appears to be performant
and is a great developer experience since it's similar to creating a web app.

## *local* Server

For the most part, the *local* server is similar to a web server except isn't bound to a port and isn't *running*. It's a
stateless function invoked by native code. It's takes two parameters. The `appContext` which holds things related to the
native app, like the native file system location for the SQLite database. The second is the `appRequest` which is an
object that represents an HTTP request. The *local* server receives the request, processes it, and returns an `appResponse`
which is an object that represents an HTTP response. The native code does whatever it needs, based on the content of the
response. The native code and *local* server behave in a way that is similar to a client-server pattern.

The *local* server framework is similar to [Actix-Web](https://actix.rs/) and actually
uses [actix-router](https://crates.io/crates/actix-router). It also leverages [Diesel](https://crates.io/crates/diesel)
with SQLite, [handlebars](https://crates.io/crates/handlebars) for rendering, and handful of other crates.

## Native Code

The native code is just the glue code between the *local* server and WebViews. For Android, it leverages a single
activity and [Fragments](https://developer.android.com/guide/fragments) to provide native-like navigation between pages.
iOS pushes a controller with a WebView on to the UINavigationController stack to handle
page navigation. When a user clicks a link or submits a form then the url and data is passed over the JavaScript
bridge to the native code and on to the *local* server. The back action pops controllers for iOS and pops fragments for
Android. It's easy to switch to native code by looking at the request coming from the JavaScript bridge and calling
native code rather than the *local* server. The native code is fairly simple, but could be as complex as the apps needs
it to be.

## Next Steps

I've figured out page navigation, forms, external HTTP requests, JavaScript, CSS, database access, and managing files
between the WebView and the native file system. The following are ideas I might pursue as my app(s) need them.

* Make the *local* server *run*. Passing the `appContext` on each request is a bit annoying. Sharing memory between the
  native file system and Rust would solve this.
* Highly interactive UIs. I could do this with JavaScript, but rather avoid it and do everything in Rust. Ideally, the
  UI would make fetch requests to the *local* server and pass the HTML response
  to [morphdom](https://github.com/patrick-steele-idem/morphdom)
  to update the DOM.
* WebSockets between the *local* server and a web server
* WebRTC between the *local* servers on different devices



