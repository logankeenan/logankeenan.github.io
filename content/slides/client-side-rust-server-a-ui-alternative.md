+++
title = "Client-Side Server w/ Rust: A UI alternative"
description = ""
date = 2019-10-17
+++


<section id="slide1">

# Client-Side Server w/ Rust: A UI alternative

Logan Keenan

<div class="slide-nav">

[next](#slide2)

</div> 
</section>



<section id="slide2">

# Server-side apps


<div>
<div style="width: 50%; float: left">
{{ resize_image(path="../static/images/typical-ssr-app.jpeg", alt="Typical Server-Side App") }}
</div>
<div style="width: calc(50% - 40px); float: left; margin-left: 40px;">

## Pros
* Fast initial page load
* Easier to develop

## Cons
* Latency
* In some cases, scaling 

</div>
</div>


<div class="slide-nav">

[previous](#slide1)
[next](#slide3)

</div>

</section>


<section id="slide3">

# Client-side apps


<div>
<div style="width: 50%; float: left">
{{ resize_image(path="../static/images/spa-app.jpeg", alt="Client-Side App") }}
</div>
<div style="width: calc(50% - 40px); float: left; margin-left: 40px;">

## Pros
* Improved UI/UX

## Cons
* Initial Page load
* Performance
* Code complexity

</div>
</div>

<div class="slide-nav">

[previous](#slide2)
[next](#slide4)

</div>
</section>


<section id="slide4">

# What if we ran the server on the client?

<div style="width: 50%; margin: 0 auto;">
{{ resize_image(path="../static/images/client-side-server-app.jpeg", alt="Client-Side Server App") }}
</div>


<div class="slide-nav"> 

[previous](#slide3)
[next](#slide5)

</div>

</section>

<section id="slide5">

# What do we need to do?

* Create an app that accepts HTTP requests and returns HTTP responses
* Compile to WASM
  * https://developer.mozilla.org/en-US/docs/WebAssembly
* Create an adapter layer to integrate the app 
  1. User clicks and anchor tag
  2. Create a request for the WASM app 
  3. call WASM app with the request
  4. receive WASM response
  5. Update the DOM with the response

**Problems**
* DOM updating
* Hijacking events
* What if the response contains javascript?
* What about old event handlers
* Routing?
* Performance?
  

<div class="slide-nav">

[previous](#slide4)
[next](#slide6)

</div>

</section>


<section id="slide6">

# Hello Service Workers 😍


<div>
<div style="width: 50%; float: left">
{{ resize_image(path="../static/images/client-side-server.jpeg", alt="Client-Side App") }}
</div>
<div style="width: calc(50% - 40px); float: left; margin-left: 40px;">

**Service Worker**: essentially act as proxy servers that sit between web applications, the browser, and the network (when available). [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Service_Worker_API) 

## Pros
* Improved UI/UX
* Improved Performance
* Easier to develop
* Fast initial page load

## Cons
* Service Worker support
* Unknown Unknowns

</div>
</div>


<div class="slide-nav">

[previous](#slide5)
[next](#slide7)
</div>

</section>

<section id="slide7">

# Code & Demo 

* [https://rust-everywhere-server-side.logankeenan.com/](https://rust-everywhere-server-side.logankeenan.com/)
* [https://rust-everywhere-spa.pages.dev/](https://rust-everywhere-spa.pages.dev/)
* [https://rust-everywhere-spa-server.logankeenan.com/](https://rust-everywhere-spa-server.logankeenan.com/)

<div class="slide-nav">

[previous](#slide6)

</div>

</section>
