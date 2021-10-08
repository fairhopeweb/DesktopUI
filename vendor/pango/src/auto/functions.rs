// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Analysis;
use crate::AttrIterator;
use crate::AttrList;
use crate::Context;
use crate::Direction;
use crate::GlyphString;
use crate::Item;
use crate::Rectangle;
use crate::Stretch;
use crate::Style;
use crate::Variant;
use crate::Weight;
use glib::translate::*;
use std::mem;
use std::ptr;

//#[cfg_attr(feature = "v1_44", deprecated = "Since 1.44")]
//#[doc(alias = "pango_break")]
//#[doc(alias = "break")]
//pub fn break_(text: &str, analysis: &mut Analysis, attrs: /*Ignored*/&[&LogAttr]) {
//    unsafe { TODO: call ffi:pango_break() }
//}

//#[doc(alias = "pango_default_break")]
//pub fn default_break(text: &str, analysis: Option<&mut Analysis>, attrs: /*Ignored*/&mut LogAttr, attrs_len: i32) {
//    unsafe { TODO: call ffi:pango_default_break() }
//}

#[doc(alias = "pango_extents_to_pixels")]
pub fn extents_to_pixels(inclusive: Option<&Rectangle>, nearest: Option<&Rectangle>) {
    unsafe {
        ffi::pango_extents_to_pixels(
            mut_override(inclusive.to_glib_none().0),
            mut_override(nearest.to_glib_none().0),
        );
    }
}

#[doc(alias = "pango_find_base_dir")]
pub fn find_base_dir(text: &str) -> Direction {
    let length = text.len() as i32;
    unsafe { from_glib(ffi::pango_find_base_dir(text.to_glib_none().0, length)) }
}

#[doc(alias = "pango_find_paragraph_boundary")]
pub fn find_paragraph_boundary(text: &str) -> (i32, i32) {
    let length = text.len() as i32;
    unsafe {
        let mut paragraph_delimiter_index = mem::MaybeUninit::uninit();
        let mut next_paragraph_start = mem::MaybeUninit::uninit();
        ffi::pango_find_paragraph_boundary(
            text.to_glib_none().0,
            length,
            paragraph_delimiter_index.as_mut_ptr(),
            next_paragraph_start.as_mut_ptr(),
        );
        let paragraph_delimiter_index = paragraph_delimiter_index.assume_init();
        let next_paragraph_start = next_paragraph_start.assume_init();
        (paragraph_delimiter_index, next_paragraph_start)
    }
}

//#[doc(alias = "pango_get_log_attrs")]
//#[doc(alias = "get_log_attrs")]
//pub fn log_attrs(text: &str, level: i32, language: &mut Language, log_attrs: /*Ignored*/&[&LogAttr]) {
//    unsafe { TODO: call ffi:pango_get_log_attrs() }
//}

#[doc(alias = "pango_is_zero_width")]
pub fn is_zero_width(ch: char) -> bool {
    unsafe { from_glib(ffi::pango_is_zero_width(ch.into_glib())) }
}

#[doc(alias = "pango_itemize")]
pub fn itemize(
    context: &Context,
    text: &str,
    start_index: i32,
    length: i32,
    attrs: &AttrList,
    cached_iter: Option<&AttrIterator>,
) -> Vec<Item> {
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::pango_itemize(
            context.to_glib_none().0,
            text.to_glib_none().0,
            start_index,
            length,
            attrs.to_glib_none().0,
            mut_override(cached_iter.to_glib_none().0),
        ))
    }
}

#[doc(alias = "pango_itemize_with_base_dir")]
pub fn itemize_with_base_dir(
    context: &Context,
    base_dir: Direction,
    text: &str,
    start_index: i32,
    length: i32,
    attrs: &AttrList,
    cached_iter: Option<&AttrIterator>,
) -> Vec<Item> {
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::pango_itemize_with_base_dir(
            context.to_glib_none().0,
            base_dir.into_glib(),
            text.to_glib_none().0,
            start_index,
            length,
            attrs.to_glib_none().0,
            mut_override(cached_iter.to_glib_none().0),
        ))
    }
}

//#[doc(alias = "pango_markup_parser_finish")]
//pub fn markup_parser_finish(context: /*Ignored*/&glib::MarkupParseContext) -> Result<(AttrList, glib::GString, char), glib::Error> {
//    unsafe { TODO: call ffi:pango_markup_parser_finish() }
//}

//#[doc(alias = "pango_markup_parser_new")]
//pub fn markup_parser_new(accel_marker: char) -> /*Ignored*/Option<glib::MarkupParseContext> {
//    unsafe { TODO: call ffi:pango_markup_parser_new() }
//}

#[doc(alias = "pango_parse_markup")]
pub fn parse_markup(
    markup_text: &str,
    accel_marker: char,
) -> Result<(AttrList, glib::GString, char), glib::Error> {
    let length = markup_text.len() as i32;
    unsafe {
        let mut attr_list = ptr::null_mut();
        let mut text = ptr::null_mut();
        let mut accel_char = mem::MaybeUninit::uninit();
        let mut error = ptr::null_mut();
        let _ = ffi::pango_parse_markup(
            markup_text.to_glib_none().0,
            length,
            accel_marker.into_glib(),
            &mut attr_list,
            &mut text,
            accel_char.as_mut_ptr(),
            &mut error,
        );
        let accel_char = accel_char.assume_init();
        if error.is_null() {
            Ok((
                from_glib_full(attr_list),
                from_glib_full(text),
                std::convert::TryFrom::try_from(accel_char)
                    .expect("conversion from an invalid Unicode value attempted"),
            ))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "pango_parse_stretch")]
pub fn parse_stretch(str: &str, warn: bool) -> Option<Stretch> {
    unsafe {
        let mut stretch = mem::MaybeUninit::uninit();
        let ret = from_glib(ffi::pango_parse_stretch(
            str.to_glib_none().0,
            stretch.as_mut_ptr(),
            warn.into_glib(),
        ));
        let stretch = stretch.assume_init();
        if ret {
            Some(from_glib(stretch))
        } else {
            None
        }
    }
}

#[doc(alias = "pango_parse_style")]
pub fn parse_style(str: &str, warn: bool) -> Option<Style> {
    unsafe {
        let mut style = mem::MaybeUninit::uninit();
        let ret = from_glib(ffi::pango_parse_style(
            str.to_glib_none().0,
            style.as_mut_ptr(),
            warn.into_glib(),
        ));
        let style = style.assume_init();
        if ret {
            Some(from_glib(style))
        } else {
            None
        }
    }
}

#[doc(alias = "pango_parse_variant")]
pub fn parse_variant(str: &str, warn: bool) -> Option<Variant> {
    unsafe {
        let mut variant = mem::MaybeUninit::uninit();
        let ret = from_glib(ffi::pango_parse_variant(
            str.to_glib_none().0,
            variant.as_mut_ptr(),
            warn.into_glib(),
        ));
        let variant = variant.assume_init();
        if ret {
            Some(from_glib(variant))
        } else {
            None
        }
    }
}

#[doc(alias = "pango_parse_weight")]
pub fn parse_weight(str: &str, warn: bool) -> Option<Weight> {
    unsafe {
        let mut weight = mem::MaybeUninit::uninit();
        let ret = from_glib(ffi::pango_parse_weight(
            str.to_glib_none().0,
            weight.as_mut_ptr(),
            warn.into_glib(),
        ));
        let weight = weight.assume_init();
        if ret {
            Some(from_glib(weight))
        } else {
            None
        }
    }
}

#[doc(alias = "pango_quantize_line_geometry")]
pub fn quantize_line_geometry(thickness: &mut i32, position: &mut i32) {
    unsafe {
        ffi::pango_quantize_line_geometry(thickness, position);
    }
}

#[doc(alias = "pango_shape")]
pub fn shape(text: &str, analysis: &Analysis, glyphs: &mut GlyphString) {
    let length = text.len() as i32;
    unsafe {
        ffi::pango_shape(
            text.to_glib_none().0,
            length,
            analysis.to_glib_none().0,
            glyphs.to_glib_none_mut().0,
        );
    }
}

//#[cfg(any(feature = "v1_44", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
//#[doc(alias = "pango_tailor_break")]
//pub fn tailor_break(text: &str, analysis: &mut Analysis, offset: i32, log_attrs: /*Ignored*/&[&LogAttr]) {
//    unsafe { TODO: call ffi:pango_tailor_break() }
//}

#[doc(alias = "pango_unichar_direction")]
pub fn unichar_direction(ch: char) -> Direction {
    unsafe { from_glib(ffi::pango_unichar_direction(ch.into_glib())) }
}

#[doc(alias = "pango_units_from_double")]
pub fn units_from_double(d: f64) -> i32 {
    unsafe { ffi::pango_units_from_double(d) }
}

#[doc(alias = "pango_units_to_double")]
pub fn units_to_double(i: i32) -> f64 {
    unsafe { ffi::pango_units_to_double(i) }
}

#[doc(alias = "pango_version")]
pub fn version() -> i32 {
    unsafe { ffi::pango_version() }
}

#[doc(alias = "pango_version_check")]
pub fn version_check(
    required_major: i32,
    required_minor: i32,
    required_micro: i32,
) -> Option<glib::GString> {
    unsafe {
        from_glib_none(ffi::pango_version_check(
            required_major,
            required_minor,
            required_micro,
        ))
    }
}

#[doc(alias = "pango_version_string")]
pub fn version_string() -> Option<glib::GString> {
    unsafe { from_glib_none(ffi::pango_version_string()) }
}