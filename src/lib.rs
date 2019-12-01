#![cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]

#[macro_use]
extern crate lazy_static;

pub mod x11_dl {
    pub use ::x11_dl::*;
}

use ::x11_dl::error::OpenError;

lazy_static! {
    pub static ref XEXT: Result<x11_dl::dpms::Xext, OpenError> = x11_dl::dpms::Xext::open();
    pub static ref XSS: Result<x11_dl::xss::Xss, OpenError> = x11_dl::xss::Xss::open();
    pub static ref XFT: Result<x11_dl::xft::Xft, OpenError> = x11_dl::xft::Xft::open();
    pub static ref XT: Result<x11_dl::xt::Xt, OpenError> = x11_dl::xt::Xt::open();
    pub static ref XMU: Result<x11_dl::xmu::Xmu, OpenError> = x11_dl::xmu::Xmu::open();
    pub static ref XRENDER: Result<x11_dl::xrender::Xrender, OpenError> =
        x11_dl::xrender::Xrender::open();
    pub static ref XCURSOR: Result<x11_dl::xcursor::Xcursor, OpenError> =
        x11_dl::xcursor::Xcursor::open();
    pub static ref GLX: Result<x11_dl::glx::Glx, OpenError> = x11_dl::glx::Glx::open();
    pub static ref XINPUT: Result<x11_dl::xinput::XInput, OpenError> =
        x11_dl::xinput::XInput::open();
    pub static ref XINPUT2: Result<x11_dl::xinput2::XInput2, OpenError> =
        x11_dl::xinput2::XInput2::open();
    pub static ref XRANDR_2_2_0: Result<x11_dl::xrandr::Xrandr_2_2_0, OpenError> =
        x11_dl::xrandr::Xrandr_2_2_0::open();
    pub static ref XRANDR: Result<x11_dl::xrandr::Xrandr, OpenError> =
        x11_dl::xrandr::Xrandr::open();
    pub static ref XF86VMODE: Result<x11_dl::xf86vmode::Xf86vmode, OpenError> =
        x11_dl::xf86vmode::Xf86vmode::open();
    pub static ref XTEST_XF86VMODE: Result<x11_dl::xtest::Xf86vmode, OpenError> =
        x11_dl::xtest::Xf86vmode::open();
    pub static ref XRECORD_XF86VMODE: Result<x11_dl::xrecord::Xf86vmode, OpenError> =
        x11_dl::xrecord::Xf86vmode::open();
    pub static ref XINERAMA_XLIB: Result<x11_dl::xinerama::Xlib, OpenError> =
        x11_dl::xinerama::Xlib::open();
    pub static ref XLIB: Result<x11_dl::xlib::Xlib, OpenError> = x11_dl::xlib::Xlib::open();
    pub static ref XLIB_XCB: Result<x11_dl::xlib_xcb::Xlib_xcb, OpenError> =
        x11_dl::xlib_xcb::Xlib_xcb::open();
}

#[macro_export]
macro_rules! syms {
    ($name:ident) => {{ glutin_x11_sym::$name.as_ref().unwrap() }};
    ($($name:ident),+) => {{( $(syms!($name)),+ )}};
}
