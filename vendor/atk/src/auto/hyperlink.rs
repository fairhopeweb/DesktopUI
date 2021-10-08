// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Action;
use crate::Object;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "AtkHyperlink")]
    pub struct Hyperlink(Object<ffi::AtkHyperlink, ffi::AtkHyperlinkClass>) @implements Action;

    match fn {
        type_ => || ffi::atk_hyperlink_get_type(),
    }
}

pub const NONE_HYPERLINK: Option<&Hyperlink> = None;

pub trait HyperlinkExt: 'static {
    #[doc(alias = "atk_hyperlink_get_end_index")]
    #[doc(alias = "get_end_index")]
    fn end_index(&self) -> i32;

    #[doc(alias = "atk_hyperlink_get_n_anchors")]
    #[doc(alias = "get_n_anchors")]
    fn n_anchors(&self) -> i32;

    #[doc(alias = "atk_hyperlink_get_object")]
    #[doc(alias = "get_object")]
    fn object(&self, i: i32) -> Option<Object>;

    #[doc(alias = "atk_hyperlink_get_start_index")]
    #[doc(alias = "get_start_index")]
    fn start_index(&self) -> i32;

    #[doc(alias = "atk_hyperlink_get_uri")]
    #[doc(alias = "get_uri")]
    fn uri(&self, i: i32) -> Option<glib::GString>;

    #[doc(alias = "atk_hyperlink_is_inline")]
    fn is_inline(&self) -> bool;

    #[doc(alias = "atk_hyperlink_is_valid")]
    fn is_valid(&self) -> bool;

    #[doc(alias = "number-of-anchors")]
    fn number_of_anchors(&self) -> i32;

    #[doc(alias = "link-activated")]
    fn connect_link_activated<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "end-index")]
    fn connect_end_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "number-of-anchors")]
    fn connect_number_of_anchors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "start-index")]
    fn connect_start_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Hyperlink>> HyperlinkExt for O {
    fn end_index(&self) -> i32 {
        unsafe { ffi::atk_hyperlink_get_end_index(self.as_ref().to_glib_none().0) }
    }

    fn n_anchors(&self) -> i32 {
        unsafe { ffi::atk_hyperlink_get_n_anchors(self.as_ref().to_glib_none().0) }
    }

    fn object(&self, i: i32) -> Option<Object> {
        unsafe {
            from_glib_none(ffi::atk_hyperlink_get_object(
                self.as_ref().to_glib_none().0,
                i,
            ))
        }
    }

    fn start_index(&self) -> i32 {
        unsafe { ffi::atk_hyperlink_get_start_index(self.as_ref().to_glib_none().0) }
    }

    fn uri(&self, i: i32) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::atk_hyperlink_get_uri(
                self.as_ref().to_glib_none().0,
                i,
            ))
        }
    }

    fn is_inline(&self) -> bool {
        unsafe { from_glib(ffi::atk_hyperlink_is_inline(self.as_ref().to_glib_none().0)) }
    }

    fn is_valid(&self) -> bool {
        unsafe { from_glib(ffi::atk_hyperlink_is_valid(self.as_ref().to_glib_none().0)) }
    }

    fn number_of_anchors(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"number-of-anchors\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `number-of-anchors` getter")
        }
    }

    fn connect_link_activated<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn link_activated_trampoline<P: IsA<Hyperlink>, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkHyperlink,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Hyperlink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"link-activated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    link_activated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_end_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_end_index_trampoline<P: IsA<Hyperlink>, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkHyperlink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Hyperlink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::end-index\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_end_index_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_number_of_anchors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_number_of_anchors_trampoline<
            P: IsA<Hyperlink>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AtkHyperlink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Hyperlink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::number-of-anchors\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_number_of_anchors_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_start_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_start_index_trampoline<
            P: IsA<Hyperlink>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AtkHyperlink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Hyperlink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::start-index\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_start_index_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Hyperlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Hyperlink")
    }
}