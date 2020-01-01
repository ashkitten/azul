#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate azul;

use azul::prelude::*;
#[cfg(debug_assertions)]
use std::time::Duration;

const TEST_IMAGE: &[u8] = include_bytes!("../../assets/images/cat_image.jpg");

struct MyDataModel;

impl Layout for MyDataModel {
    fn layout(&self, info: LayoutInfo) -> Dom<Self> {
        Dom::div().with_id("wrapper")
            .with_child(Dom::label("Hello123").with_id("red"))
            .with_child(Dom::div().with_id("sub-wrapper")
                .with_child(Dom::div().with_id("yellow")
                    .with_child(Dom::div().with_id("orange-1"))
                    .with_child(Dom::div().with_id("orange-2"))
                )
                .with_child(Dom::div().with_id("grey"))
            )
            .with_child(Dom::image(*info.resources.get_css_image_id("Cat01").unwrap()).with_id("cat"))
            .with_child((0..50).map(|i| Dom::label(format!("{}", i))).collect::<Dom<Self>>().with_id("rows"))
    }
}

fn main() {

    macro_rules! CSS_PATH { () => (concat!(env!("CARGO_MANIFEST_DIR"), "/../examples/hot_reload/hot_reload.css")) }

    let mut app = App::new(MyDataModel, AppConfig::default()).unwrap();
    let image_id = app.resources.add_css_image_id("Cat01");
    app.resources.add_image_source(image_id, ImageSource::Embedded(TEST_IMAGE));

    #[cfg(debug_assertions)]
    let window = WindowCreateOptions::new_hot_reload(css::hot_reload_override_native(CSS_PATH!(), Duration::from_millis(500)));

    #[cfg(not(debug_assertions))]
    let window = WindowCreateOptions::new(css::override_native(include_str!(CSS_PATH!())).unwrap());

    app.run(window);
}
