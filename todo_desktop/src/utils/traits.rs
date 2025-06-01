#![allow(dead_code)]

use glib::IsA;
use gtk::{
    pango::ffi::PANGO_SCALE, prelude::{DialogExtManual, GtkWindowExt, ImageExt, StyleContextExt, WidgetExt}, traits::{ContainerExt, HeaderBarExt}, IconSize, ResponseType, Widget
};

use super::Size;

pub trait WidgetExt2 {
    fn set_request_size(&self, size: Size);

    fn set_classes(&self, classes: &[&str]);

    fn set_font_family(&self, family: &str);

    fn set_font_size(&self, size: u32);
}

impl<W: WidgetExt> WidgetExt2 for W {
    fn set_request_size(&self, size: Size) {
        self.set_size_request(size.0 as i32, size.1 as i32);
    }
    
    fn set_classes(&self, classes: &[&str]) {
        let ctx = self.style_context();

        for class in classes {
            ctx.add_class(class);
        }
    }

    fn set_font_family(&self, family: &str) {
        let ctx = self.pango_context();

        let mut font = ctx.font_description().unwrap_or_default();
        font.set_family(family);
        ctx.set_font_description(Some(&font));
    }
    
    fn set_font_size(&self, size: u32) {
        let ctx = self.pango_context();

        let mut font = ctx.font_description().unwrap_or_default();
        font.set_size(size.max(1) as i32 * PANGO_SCALE);
        ctx.set_font_description(Some(&font));
    }
}

pub trait WindowExt2 {
    fn set_size(&self, size: Size);
}

impl<W: GtkWindowExt> WindowExt2 for W {
    fn set_size(&self, size: Size) {
        self.set_default_size(size.0 as i32, size.1 as i32);
    }
}

pub trait ImageExt2 {
    fn set_icon(&self, name_size: (&str, IconSize));
}

impl<I: ImageExt> ImageExt2 for I {
    fn set_icon(&self, name_size: (&str, IconSize)) {
        self.set_from_icon_name(Some(name_size.0), name_size.1);
    }
}

pub trait DialogExt2 {
    fn set_buttons(&self, buttons: &[(&str, ResponseType)]);
}

impl<D: DialogExtManual> DialogExt2 for D {
    fn set_buttons(&self, buttons: &[(&str, ResponseType)]) {
        self.add_buttons(buttons);
    }
}

pub trait HeaderBarExt2 {
    fn set_start_child(&self, child: &impl IsA<Widget>);

    fn set_end_child(&self, child: &impl IsA<Widget>);
}

impl<H: HeaderBarExt> HeaderBarExt2 for H {
    fn set_start_child(&self, child: &impl IsA<Widget>) {
        self.pack_start(child);
    }

    fn set_end_child(&self, child: &impl IsA<Widget>) {
        self.pack_end(child);
    }
}

pub trait ContainerExt2 {
    fn set_content(&self, child: Option<&impl IsA<Widget>>);
}

impl<C: ContainerExt> ContainerExt2 for C {
    fn set_content(&self, child: Option<&impl IsA<Widget>>) {
        let children = self.children();

        for child in children {
            self.remove(&child);
        }

        if let Some(child) = child {
            self.add(child);
        }
    }
}