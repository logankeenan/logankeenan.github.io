+++
title = "Rendering Handlebar Templates with Rust and WASM"
description = "Rendering Handlebar Templates with Rust and WASM"
date = 2020-06-24
+++


The Rust implementation of <a target="_blank" href="https://github.com/sunng87/handlebars-rust">Handlebars</a>
was easy to use on a server, so I wanted
to see what it would take to render templates in the browser. I'm going to demonstration two features for
Handlebars. However, it's not limited to these. Lets take a look at using partials and helpers.
    
## Handlebar Partials

We'll leverage partials to create a page layout and nest a page within it.
The code was basically the same as the server. However, the templates cannot be read from
the file system in the browser. Thankfully, Rust has the <a target="_blank"
                                             href="https://doc.rust-lang.org/std/macro.include_str.html">include_str!</a>
macro which allows the templates to be read at compile time and included in the binary.

```handlebars 
<section>
    <h1>Plantings 2020</h1>
    {{> page_content}}
</section>
```
```handlebars
{{#> layout_page}}
    {{#*inline "page_content"}}
        <ul>
            {{#each plants}}
                <li>
                    {{this}}
                </li>
            {{/each}}
        </ul>
    {{/inline}}
{{/layout_page}}
```

```rust
let layout = include_str!("./templates/layout.hbs");
let plantings_2020 = include_str!("templates/plantings_list.hbs");

let mut handlebars = Handlebars::new();
handlebars.register_template_string("layout_page", layout).unwrap();
let result = handlebars.render_template(
    plantings_2020,
    &json!({"plants":["green beans","tomatoes","peas","zucchini","peppers","cucumbers","soy beans","corn","melons"]}),
);
```

##  Handlebar Helpers

Fortunately, custom helpers is the same as on the server.

```rust
handlebars_helper!(uppercase: | string_to_uppercase: str| {
    string_to_uppercase.to_uppercase()
});
handlebars.register_helper("uppercase", Box::new(uppercase));
```
    
## Next Steps
    
Check out the demo <a href="https://github.com/logankeenan/logankeenan.github.io/repos/wasm-handlebars">repository</a>
to see the full code as well as a working <a href="/demos/wasm-handlebars/index.html">demo</a>.
The demo simply calls the Rust code (compiled to Wasm) which returns some html markup which is set to the
document body.
    
This solution doesn't seem all that developer friendly. Developers would have to remember to include any new template
so they'll be contained in the binary. The <a target="_blank" href="https://crates.io/crates/include_dir">include_dir</a>
crate could allow for the creation of a template factory using path with filename as a parameter for a given template.
    