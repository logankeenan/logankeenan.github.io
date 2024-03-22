+++
title = "Local Internet Archive w/ Ditto"
description = "Explore the Local Internet Archive, a proof-of-concept app that leverages Ditto's networking technology to save and share web content locally. Ideal for communities with limited internet access, this app serves as a localized internet resource."
date = 2023-09-10
+++

I've always been fascinated by the idea of networking personal devices together to form a distributed system. What kinds
of problems can be solved when these devices can talk to each other, even without an internet connection? This question
becomes especially relevant considering the millions of people around the world who lack consistent internet access at
home. One challenge that comes to mind is how to save and share web content for later use when you're offline. To tackle
this, I've developed a straightforward proof-of-concept app using [Ditto](https://www.ditto.live/)'s robust networking technology.
<style>
.full-width-video {
  width: 100%;
  height: auto;
  aspect-ratio: 16 / 9; /* Maintain the aspect ratio */
}
</style>   
<iframe class="full-width-video" width="560" height="315" src="https://www.youtube.com/embed/hGB1pURveVU?si=tbhylv6fwstjBxHz" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

## What is Ditto?

Ditto is a software development kit (SDK) and platform that enables real-time data synchronization across devices
connected via Bluetooth, Wi-Fi Direct, LAN, or the Internet.

## Use Case

Imagine you work in a city but return to a small community with no internet access each night. With this app, you
can save valuable websites to your device and access them later at home. Moreover, community members can view and
download these saved websites onto their own devices, even without an internet connection. This not only allows for
collective aggregation of valuable information but also serves as a localized internet resource for the community.

[Source Code](https://github.com/logankeenan/local-internet-archive/)
