mod transfer;
use io_kit_sys::ret::{
    kIOReturnAborted, kIOReturnNoDevice, kIOReturnSuccess, kIOReturnUnderrun, IOReturn,
};
pub(crate) use transfer::TransferData;

mod enumeration;
mod events;
pub use enumeration::list_devices;

mod device;
pub(crate) use device::MacDevice as Device;
pub(crate) use device::MacInterface as Interface;

use crate::transfer::TransferError;

mod iokit;
mod iokit_c;
mod iokit_usb;

fn status_to_transfer_result(status: IOReturn) -> Result<(), TransferError> {
    #[allow(non_upper_case_globals)]
    #[deny(unreachable_patterns)]
    match status {
        kIOReturnSuccess | kIOReturnUnderrun => Ok(()),
        kIOReturnNoDevice => Err(TransferError::Disconnected),
        kIOReturnAborted => Err(TransferError::Cancelled),
        _ => Err(TransferError::Unknown),
    }
}
