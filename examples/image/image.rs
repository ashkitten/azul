use azul::{prelude::*, widgets::button::Button};

struct GlitchApp {}

impl Layout for GlitchApp {
    fn layout(&self, info: LayoutInfo<Self>) -> Dom<Self> {
        let button = Button::with_label("Load Image")
            .dom()
            .with_callback(On::MouseUp, load_image);

        let image = if let Some(image_id) = info.resources.get_css_image_id("preview_image") {
            Dom::image(*image_id)
        } else {
            Dom::div()
        };

        Dom::div().with_child(button).with_child(image)
    }
}

fn load_image(info: CallbackInfo<GlitchApp>) -> UpdateScreen {
    if let Some(path) = azul::dialogs::open_file_dialog(None, None) {
        let image_id = info.state.resources.add_css_image_id("preview_image");
        info.state
            .resources
            .add_image_source(image_id, ImageSource::File(path.into()));
    }
    Redraw
}

fn main() {
    let mut app = App::new(GlitchApp {}, AppConfig::default()).unwrap();
    let window = app
        .create_window(WindowCreateOptions::default(), css::native())
        .unwrap();
    app.run(window).unwrap();
}