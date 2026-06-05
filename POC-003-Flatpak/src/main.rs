use ashpd::desktop::screenshot::Screenshot;
use adw::prelude::*;
use gio::File;
use glib::ControlFlow;
use gtk4 as gtk;
use libadwaita as adw;
use std::path::PathBuf;
use url::Url;

const APP_ID: &str = "io.github.screenshothero.Poc003";
const WINDOW_TITLE: &str = "Screenshot Hero POC 003-04";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &adw::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title(WINDOW_TITLE)
        .default_width(1000)
        .default_height(700)
        .build();

    let header = adw::HeaderBar::new();
    let take_button = gtk::Button::with_label("Take Screenshot");
    header.pack_start(&take_button);

    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    content.append(&header);

    let status_label = gtk::Label::new(Some("Click 'Take Screenshot' to capture and preview."));
    status_label.set_xalign(0.0);
    content.append(&status_label);

    let picture = gtk::Picture::new();
    picture.set_hexpand(true);
    picture.set_vexpand(true);
    picture.set_can_shrink(true);
    picture.set_keep_aspect_ratio(true);
    content.append(&picture);

    window.set_content(Some(&content));
    window.present();

    let (sender, receiver) = std::sync::mpsc::channel::<Result<PathBuf, String>>();

    let take_button_for_result = take_button.clone();
    let status_label_for_result = status_label.clone();
    let picture_for_result = picture.clone();

    glib::idle_add_local(move || {
        while let Ok(result) = receiver.try_recv() {
            take_button_for_result.set_sensitive(true);

            match result {
                Ok(path) => {
                    if path.exists() {
                        let file = File::for_path(path);
                        picture_for_result.set_file(Some(&file));
                        status_label_for_result.set_text("Screenshot captured successfully.");
                    } else {
                        status_label_for_result.set_text("Could not open the captured image file.");
                    }
                }
                Err(message) => status_label_for_result.set_text(&message),
            }
        }

        ControlFlow::Continue
    });

    let status_label_for_click = status_label.clone();
    take_button.connect_clicked(move |button| {
        button.set_sensitive(false);
        status_label_for_click.set_text("Waiting for screenshot selection...");

        let sender_for_thread = sender.clone();
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build();

            let result = match runtime {
                Ok(rt) => rt.block_on(capture_screenshot_path()),
                Err(error) => Err(format!("Internal runtime initialization failed: {error}")),
            };

            if sender_for_thread.send(result).is_err() {
                eprintln!("Warning: could not deliver screenshot result to UI.");
            }
        });
    });
}

async fn capture_screenshot_path() -> Result<PathBuf, String> {
    let request = Screenshot::request()
        .interactive(true)
        .modal(true)
        .send()
        .await;
    let response = match request {
        Ok(r) => r,
        Err(error) => return Err(map_portal_error(error.to_string())),
    };

    let response = response
        .response()
        .map_err(|error| map_portal_error(error.to_string()))?;

    let uri = response.uri();

    let parsed_uri =
        Url::parse(uri.as_str()).map_err(|_| "Received an invalid screenshot URI.".to_string())?;

    parsed_uri
        .to_file_path()
        .map_err(|_| "Could not convert screenshot URI to a file path.".to_string())
}

fn map_portal_error(error_text: String) -> String {
    if error_text.to_lowercase().contains("cancel") {
        "Screenshot capture was canceled.".to_string()
    } else {
        format!("Screenshot capture failed: {error_text}")
    }
}
