mod device;
mod enumeration;
mod hotplug;
mod transfer;

use std::io::Error;

pub(crate) use transfer::TransferData;

pub use enumeration::{list_buses, list_devices};

pub(crate) use device::WebusbDevice as Device;
pub(crate) use device::WebusbInterface as Interface;
pub(crate) use hotplug::WebusbHotplugWatch as HotplugWatch;

use web_sys::js_sys;
use web_sys::js_sys::Reflect;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::JsValue;
use web_sys::Usb;
use web_sys::UsbDevice;
use web_sys::Window;
use web_sys::WorkerGlobalScope;

use crate::transfer::TransferError;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DeviceId {
    pub(crate) id: usize,
}

impl DeviceId {
    pub(crate) fn from_device(device: &UsbDevice) -> Self {
        let key = JsValue::from_str("nusbUniqueId");
        static INCREMENT: std::sync::LazyLock<std::sync::Mutex<usize>> =
            std::sync::LazyLock::new(|| std::sync::Mutex::new(0));
        let id = if let Ok(device_id) = Reflect::get(device, &key) {
            device_id
                .as_f64()
                .expect("Expected an integer ID. This is a bug. Please report it.")
                as usize
        } else {
            let mut lock = INCREMENT
                .lock()
                .expect("this should never be poisoned as we do not have multiple threads");
            *lock += 1;
            Reflect::set(device, &key, &JsValue::from_f64(*lock as f64))
                .expect("Could not set ID on JS object. This is a bug. Please report it.");
            *lock
        };

        DeviceId { id }
    }
}

pub(crate) fn web_to_nusb_status(status: web_sys::UsbTransferStatus) -> Result<(), TransferError> {
    match status {
        web_sys::UsbTransferStatus::Ok => Ok(()),
        web_sys::UsbTransferStatus::Stall => Err(TransferError::Stall),
        web_sys::UsbTransferStatus::Babble => Err(TransferError::Unknown),
        _ => unreachable!(),
    }
}

pub(crate) fn usb() -> Result<Usb, Error> {
    let window = js_sys::global().dyn_into::<Window>().ok();

    if let Some(window) = window {
        return Ok(window.navigator().usb());
    }

    let wgs = js_sys::global().dyn_into::<WorkerGlobalScope>().ok();

    if let Some(wgs) = wgs {
        return Ok(wgs.navigator().usb());
    }

    Err(Error::other("WebUSB is not available on this platform"))
}
