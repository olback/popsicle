use super::View;
use gtk::*;
use pango::{AttrList, Attribute, EllipsizeMode};
use std::path::Path;

pub struct ImageView {
    pub view: View,
    pub check: Button,
    pub chooser_container: Stack,
    pub chooser: Button,
    pub image_path: Label,
    pub hash: ComboBoxText,
    pub hash_label: Entry,
}

impl ImageView {
    pub fn new() -> ImageView {
        let chooser = cascade! {
            Button::new_with_label("Choose Image");
            ..set_halign(Align::Center);
            ..set_margin_bottom(6);
        };

        let image_path = cascade! {
            Label::new("No image selected");
            ..set_ellipsize(EllipsizeMode::End);
            ..get_style_context().add_class("bold");
        };

        let button_box = cascade! {
            Box::new(Orientation::Vertical, 0);
            ..pack_start(&chooser, false, false, 0);
            ..pack_start(&image_path, false, false, 0);
        };

        let spinner = Spinner::new();
        spinner.start();

        let spinner_label = cascade! {
            Label::new("Generating Checksum");
            ..get_style_context().add_class("bold");
        };

        let spinner_box = cascade! {
            Box::new(Orientation::Vertical, 0);
            ..pack_start(&spinner, false, false, 0);
            ..pack_start(&spinner_label, false, false, 0);
        };

        let hash = cascade! {
            ComboBoxText::new();
            ..append_text("Type");
            ..append_text("SHA256");
            ..append_text("MD5");
            ..set_active(0);
            ..set_sensitive(false);
        };

        let hash_label = cascade! {
            Entry::new();
            ..set_sensitive(false);
        };

        let label = cascade! {
            Label::new("Hash:");
            ..set_margin_end(6);
        };

        let check = cascade! {
            Button::new_with_label("Check");
            ..get_style_context().add_class(&STYLE_CLASS_SUGGESTED_ACTION);
            ..set_sensitive(false);
        };

        let combo_container = cascade! {
            Box::new(Orientation::Horizontal, 0);
            ..add(&hash);
            ..pack_start(&hash_label, true, true, 0);
            ..get_style_context().add_class("linked");
        };

        let hash_container = cascade! {
            tmp: Box::new(Orientation::Horizontal, 0);
            ..pack_start(&label, false, false, 0);
            ..pack_start(&combo_container, true, true, 0);
            ..pack_start(&check, false, false, 0);
            ..set_border_width(6);
        };

        let chooser_container = cascade! {
            Stack::new();
            ..add_named(&button_box, "chooser");
            ..add_named(&spinner_box, "checksum");
            ..set_visible_child_name("chooser");
            ..set_margin_top(12);
            ..set_margin_bottom(24);
        };

        let view = View::new(
            "application-x-cd-image",
            "Choose an Image",
            "Select the .iso or .img that you want to flash. You can also plug your USB drives in \
             now.",
            |right_panel| {
                right_panel.pack_start(&chooser_container, true, false, 0);
                right_panel.pack_start(&hash_container, false, false, 0);
            },
        );

        ImageView { view, check, chooser_container, chooser, image_path, hash, hash_label }
    }

    pub fn set_hash_sensitive(&self, sensitive: bool) {
        self.hash.set_sensitive(sensitive);
        self.check.set_sensitive(sensitive);
        self.hash_label.set_sensitive(sensitive);
    }

    pub fn set_hash(&self, hash: &str) {
        if let Some(text) = self.hash_label.get_text().filter(|text| !text.is_empty()) {
            if let Some(fg) = if text == hash {
                Attribute::new_foreground(0, std::u16::MAX, 0)
            } else {
                Attribute::new_foreground(std::u16::MAX, 0, 0)
            } {
                let attrs = AttrList::new();
                attrs.insert(fg);
                self.hash_label.set_attributes(&attrs);
            }
        } else {
            self.hash_label.set_text(hash);
        }
    }

    pub fn set_image_path(&self, path: &Path) {
        self.image_path.set_label(
            &path.file_name().expect("file chooser can't select directories").to_string_lossy(),
        );
    }
}
