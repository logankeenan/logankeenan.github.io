#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate handlebars;

use wasm_bindgen::prelude::*;
use handlebars::Handlebars;

handlebars_helper!(uppercase: | string_to_uppercase: str| {
        string_to_uppercase.to_uppercase()
});

#[wasm_bindgen]
pub fn render() -> String {
    let layout = include_str!("./templates/layout.hbs");
    let plantings_2020 = include_str!("templates/plantings_list.hbs");

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("layout_page", layout).unwrap();
    let result = handlebars.render_template(
        plantings_2020,
        &json!({"plants":["green beans","tomatoes","peas","zucchini","peppers","cucumbers","soy beans","corn","melons"]}),
    );

    handlebars.register_helper("uppercase", Box::new(uppercase));


    result.unwrap()
}