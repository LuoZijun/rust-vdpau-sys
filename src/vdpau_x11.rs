use crate::libc::c_int;
use crate::x11::xlib::{ Display, Drawable, };

use crate::vdpau::{
    VdpStatus,
    VdpDevice, VdpGetProcAddress,
    VdpPresentationQueueTarget,
};


/// \brief Create a VdpDevice object for use with X11.
/// \param[in] display The X Display that the VdpDevice VdpDevice will operate against.
/// \param[in] screen The X screen that the VdpDevice will operate against.
/// \param[out] device The new device's handle.
/// \param[out] get_proc_address The get_proc_address entry point to use with this device.
/// \return VdpStatus The completion status of the operation.
pub type VdpDeviceCreateX11 = Option<
    unsafe extern "C" fn(
        display: *mut Display,
        screen: c_int,
        device: *mut VdpDevice,
        get_proc_address: *mut *mut VdpGetProcAddress
    ) -> VdpStatus,
>;

/// \brief Create a VdpPresentationQueueTarget for use with X11.
/// \param[in] device The device that will contain the queue target.
/// \param[in] drawable The X11 Drawable that the presentation queue will present into.
/// \param[out] target The new queue target's handle.
/// \return VdpStatus The completion status of the operation.
/// 
/// Note: VDPAU expects to own the entire drawable for the duration of time
/// that the presentation queue target exists. In particular,
/// implementations may choose to manipulate client-visible X11 window state
/// as required. As such, it is recommended that applications create a
/// dedicated window for the presentation queue target, as a child
/// (grand-child, ...) of their top-level application window.
/// 
/// Applications may also create child-windows of the presentation queue
/// target, which will cover any presented video in the normal fashion. VDPAU
/// implementations will not manipulate such child windows in any fashion.
pub type VdpPresentationQueueTargetCreateX11 = Option<
    unsafe extern "C" fn(
        device: VdpDevice,
        drawable: Drawable,
        target: *mut VdpPresentationQueueTarget,
    ) -> VdpStatus,
>;


#[link(name="vdpau", kind="dylib")]
extern "C" {
    /// \brief Create a VdpDevice object for use with X11.
    /// This is an actual symbol of type \ref VdpDeviceCreateX11
    pub static vdp_device_create_x11: VdpDeviceCreateX11;
}

