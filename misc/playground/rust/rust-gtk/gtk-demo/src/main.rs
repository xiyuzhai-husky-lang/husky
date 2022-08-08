use gtk::prelude::*;
use gtk::Application;

fn main() {
    let app = Application::builder()
        .application_id("com.jwestall.ui-demo")
        .build();

    app.run();
}
