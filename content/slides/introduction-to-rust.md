+++
title = "Introduction to Rust"
description = "Instruction to the Rust programming language talk slides"
date = 2019-10-17
+++


<section id="slide1">

# An Introduction to the Rust Programming Language

Logan Keenan

<div class="slide-nav">

[next](#slide2)

</div> 
</section>



<section id="slide2">

# A Brief History

* Started by Graydon Hoare  
* Adopted by Mozilla
* 1.0 release in 2015

<!-- 
# Dictated Notes:
 
Started out as a project by Gordon in 2006. Mozilla adopted the project to use on the new servo browser engine.  At 
the time, even with great developers and tools, it was hard to write C/C++ code that was safe and secure.  The language
significantly evolved until the 1.0 release in 2015.  They used to have green threads instead of system threads and 
many other language feature that are well established in other languages.  The time before 1.0 was a time of experimentation
where Rust borrowed the best features of other languages.

New releases every 6 weeks. They've taken the CI approach to language development and ensure backwards compatibility 
since 1.0
-->

<div class="slide-nav">

[previous](#slide1)
[next](#slide3)

</div>

</section>


<section id="slide3">

# What is Rust and how is it different?

* All purpose modern programming language
* Zero-cost abstractions
* Fast
    * No Runtime
    * No Garbage collector
* Safe
    * Memory safe
    * Thread safe
* Interoperable
    * C/C++, Swift, Java, Ruby, Erlang, Node.js, etc...

<!-- 
Dictated Notes: 
* Rust is an all purpose modern programming lanagues.  It's borrowed a lot of good ideas from other languages. 
It's still often referred to as a systems programming language because you can use it as systems programming language
and have that fine grained control needed for OS, embedded systems, and critical software. However it can be used for 
web servers, native apps, WASM, and anything else you'd use for any other programming programming language
* zero cost abstractions
    What do I mean by zero cost extra abstractions? In its simplest definition, you don't pay a penalty for having nice things. 
    An example of this would be using generics. Generics can be used to help reduce the amount of code that you need 
    along with other benefits. However most languages have a performance penalty for using them but the cost of the penalty 
    is less than the cost of not using generics.  Rasta have really nice features without having to pay for those with a 
    performance penalty. One thing that took a long time to make it into the rust library was async. they wanted to make 
    sure that using async was a real zero cost abstraction.
* fast 
    * Comparable to C or C++ in speed
    * Rust basically has no runtime. Or at least in the sense that you think of with other languages. So what do I mean 
        by runtime. This is obviously a little simplified but a runtime is just the environment in which your program runs. 
        If I have a node ruby or python program that I want to execute then I need to pass it to the appropriate runtime.
        I have to install no Ruby or python or Java on my server in order for my program to actually execute work. 
        The run time takes the program and execute it and I see the result.  Rest doesn't have a runtime and everything 
        that's needed for that program to run is included in the executable so all you have to do is run the execuable 
        and see the progress results.  So what does this mean? It makes Rust great candidate for a embedded systems and 
        also means that Rust boot pretty much instantly.  The overall size of the program is going to be smaller 
        because you're only compiling what you need and the other things that are commonly used like in a runtime.  
    * Rust has no garbage collector and it solves the problem by this idea of "ownership" which will talk about a little bit later. 
        Even though it doesn't have a garbage collector You don't need to manage memory like he would in C or C++.  The 
        compiler forces you to write code in a way that won't allow the program to access something that doesn't exist 
        in memory memory issue.  I'll show you a working example later in the talk.  So what are the implications of 
        not having a garbage collector? Your code has a predictable memory footprint. Also resources aren't being taken 
        away from the execution of the program to the execution of the garbage collector which your program faster.
* Safe 
    * You can write Rust cost that will never have a memory issue.  I won't have some dangling pointer, free after use 
     or any other undefined behavior caused by memory.  The compiler in force is this compile time.  What this means is 
     that you know how to write a ton of defensive coach which is also really nice and can make your code a lot smaller 
     and nicer and easy to read.  Microsoft 70% of all security bugs are related to memory safety which is why they're
     starting to invest in Rust.  This might not seem like an issue that is relevant to what developers but if you've 
     ever had no point exceptions undefined errors or whatever relating to something that you thought was going to be
     there but isn't then Rust can help solve those problems.      
    * Russ describes itself as having fearless concurrency.  Because of the ownership model which will talk about a 
     little bit you can right rust code is guaranteed to be tread safe. You won't have threads accessing or changing 
     memory in an unpredictable way.  Again this is in force by the compiler.
* It's a very interoperable with other languages.  This means that you can call rest from some language and some 
    language can call rust. Which can make it really nice if your main application is some of the language and then you 
    need need better performance then you can write it in Rust.  Rust compiles down to machine code. One implication of
    that means you can write Rust code that will run on both iOS and Android in a truly native way.    
* there are other ways in which rust is different, but these are the main points.

     
-->

<div class="slide-nav">

[previous](#slide2)
[next](#slide4)

</div>
</section>


<section id="slide4">

# Downsides

* Steep learning curve
* Long compile times

<!-- 
Dictated Notes: 
* Some of the Rust paradigms can be difficult to grasp at first, especially ownership. Those that come from a C background will probably 
pick up rust more quickly.  However once you've grabbed some of these paradigms and patterns rest is very nice to work with.
* Compared to other languages the compiler is slower. That's because the compiler does a lot more than other languages 
 as we discussed with some of the other problems that solves the other languages have.
  
-->

<div class="slide-nav"> 

[previous](#slide3)
[next](#slide5)

</div>

</section>

<section id="slide5">

# Getting Started

* Install [Rust](https://www.rust-lang.org/tools/install)
* "Ownership"
* Structs & Traits

<!-- 
Dictated Notes: 
rustup: Rest up is just basically your version manager for installing rust so you can think of this like in DM or our BM it'll help you install versions of rust and then also compile targets
rustc: 
cargo: package manager

Things to show
* rustup
* rustc 
* cargo
* cargo new
* take a look at what it creates
* debug 
    * quicker than release
* release

-->

<div class="slide-nav">

[previous](#slide4)
[next](#slide6)

</div>

</section>


<section id="slide6">

# Resources

* Free Book: [The Rust Programming Language](https://doc.rust-lang.org/book/)
* YouTube: [Jon Gjengset](https://www.youtube.com/channel/UC_iD0xppBwwsrM9DegC5cQQ)
* YouTube: [Ryan Levick](https://www.youtube.com/channel/UCpeX4D-ArTrsqvhLapAHprQ)
* [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
* Rust libraries for web development  [arewewebyet.org](https://www.arewewebyet.org/)
* Mailing List: [This Week in Rust](https://this-week-in-rust.org/)
* Awesome List: [Awesome Rust](https://github.com/rust-unofficial/awesome-rust)
<!-- 

-->

<div class="slide-nav">

[previous](#slide5)

</div>

</section>
