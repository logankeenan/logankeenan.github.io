+++
title = "Rendering HTML with Rust using Components"
description = "Rendering HTML with Rust using Components"
date = 2019-12-07
+++
    
In my last Rust post I decided to use Handlebars for the view layer. Handlerbars is really simple and easy to
get going, but I wanted something a little more powerful. Awhile ago, I came across a Rust library called 
[typed-html](https://github.com/bodil/typed-html) created by [Bodil Stokke](https://twitter.com/bodil). It's a fantastic library. What's so great
about it? Besides the JSX like syntax, it'll only produce valid html and everything compiles! The
biggest advantage I see with this is being able to change my attributes on my models and having the compiler
tell me that my views are still correct. I haven't had the chance to work with a view layer with so much
safety and I'm very much looking forward to it.

First, I want to make sure I can create components to reuse my html code. Without that, I'd have to look for
another view layer. After a little messing around I was able to get some code working. My simple example is to
create a function where given two string arguments will render a button. The two string arguments represent
the class name and the text of the button. Then I created two other functions to create a primary button and
secondary button. Those two functions only argument is the button text. The functions call the button
function and provide a class name specific to the caller function. This allows the class name to be encapsulated
within the primary and secondary button functions. It's a pretty simple example, but can be
extrapolated to more complex components solutions. The code is posted below and a link to the source code is at
the bottom of the page.

Overall, I thought it worked pretty well. I'm using Intellij and would really like code completion inside the
<code>html!</code> macro. Code completion isn't a necessity, but it'd be really nice to have while I'm still
learning Rust. 
sorry.

```rust
use typed_html::dom::DOMTree;
use typed_html::html;
use typed_html::text;
use typed_html::elements::FlowContent;


fn button(class: &str, button_text: &str) -> Box<dyn FlowContent<String>> {
    html!( <button class={class} type="button"> {text!(button_text)} </button>)
}

fn primary_button(button_text: &str) -> Box<dyn FlowContent<String>> {
    button("Primary", button_text)
}

fn secondary_button(button_text: &str) -> Box<dyn FlowContent<String>> {
    button("Secondary", button_text)
}


fn layout() -> String {
    let doc: DOMTree<String> = html!(
    <html>
        <head>
            <title>"typed-html-components"</title>
        </head>
        <body>
            {primary_button("Click Primary!")}
            {secondary_button("Click Secondary!")}
        </body>
    </html>
);
    return doc.to_string();
}

fn main() {
    println!("{}", layout());
}
```

[Source Code](https://github.com/logankeenan/typed-html-components)