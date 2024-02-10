mod device;
mod enumeration;
mod hotplug;
mod transfer;

pub(crate) use transfer::TransferData;

pub use enumeration::{list_buses, list_devices};

pub(crate) use device::WebusbDevice as Device;
pub(crate) use device::WebusbInterface as Interface;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DeviceId {}

pub(crate) use hotplug::WebusbHotplugWatch as HotplugWatch;
