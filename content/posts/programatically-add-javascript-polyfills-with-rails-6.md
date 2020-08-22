
+++
title = "Programmatically add Javascript Polyfills with Rails 6"
description = "Programmatically add Javascript Polyfills with Rails 6 to prevent loading unneeded javascript files."
date = 2019-10-25
+++



Including polyfills in the application.js file forces users who have the most update-to-date browsers to download,
parse, and executed javascript they do not need. Programmatically fetching polyfills allows the application.js
file to be as small as possible; reducing parse and execution times of Javascript.

## Including the Polyfills as packs.
  
In this example, where going to setup polyfills for Promise and window.fetch. We'll start by adding a polyfill
from npm.
  
```bash
yarn add whatwg-fetch promise-polyfill
```

Create a pack for the promise polyfill at /app/javascript/packs/polyfills/promise.js
    
```js
import 'promise-polyfill/src/polyfill'
```

Create a pack for the fetch polyfill at /app/javascript/packs/polyfills/fetch.js

```js 
import 'whatwg-fetch'    
```

        
## Adding the polyfills to the application.html.erb file
    
For most cases, the polyfills need to be loaded above the application.js file because it might need to use them
during application.js execution. We'll create a script that will be executed as the page is loaded and if a
polyfill does not exist then it'll add the appropriate polyfill script file. The polyfill will be
downloaded, parsed, and executed before the application.js.
    
```html
<head>
    <!-- other head content -->

    <script>
        window.fetch || document.write('<script src="<%= asset_pack_path('polyfills/fetch.js') %>"><\/script>');
        window.Promise || document.write('<script src="<%= asset_pack_path('polyfills/promise.js') %>"><\/script>');
    </script>
    <%= javascript_pack_tag 'application', 'data-turbolinks-track': 'reload' %>

    <!-- other head content -->
</head>   
```
[Source Code](https://github.com/logankeenan/programmatically-add-javascript-polyfills-rails-6)