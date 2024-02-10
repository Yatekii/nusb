use std::{sync::Arc, time::Duration};

use wasm_bindgen_futures::{js_sys::Array, wasm_bindgen::JsCast, JsFuture};
use web_sys::UsbDevice;

use crate::{
    transfer::{Control, Direction, EndpointType, TransferError, TransferHandle},
    DeviceInfo, Error,
};

pub(crate) struct WebusbDevice {
    device: UsbDevice,
}

/// SAFETY: This is NOT safe at all.
unsafe impl Sync for WebusbDevice {}
unsafe impl Send for WebusbDevice {}

impl WebusbDevice {
    pub(crate) async fn from_device_info(d: &DeviceInfo) -> Result<Arc<WebusbDevice>, Error> {
        let window = web_sys::window().unwrap();
        let navigator = window.navigator();
        let usb = navigator.usb();
        let devices = JsFuture::from(usb.get_devices()).await.unwrap();

        let devices: Array = JsCast::unchecked_from_js(devices);

        for device in devices {
            let device: UsbDevice = JsCast::unchecked_from_js(device);
            tracing::info!("{:x?}", device.vendor_id());
            tracing::info!("{:x?}", device.product_id());
            tracing::info!("{:?}", device.serial_number());
            tracing::info!("{:#?}", d);
            if device.vendor_id() == d.vendor_id
                && device.product_id() == d.product_id
                && device.serial_number() == d.serial_number
            {
                JsFuture::from(device.open()).await.unwrap();
                return Ok(Arc::new(Self { device }));
            }
        }
        Err(Error::other("device not found"))
    }

    pub(crate) fn handle_events(&self) {
        todo!()
    }

    pub(crate) fn configuration_descriptors(&self) -> impl Iterator<Item = &[u8]> {
        let descriptors = self.device.configurations();
        descriptors.into_iter().map(|d| {
            let descriptor: UsbDevice = JsCast::unchecked_from_js(d);
            descriptor;
            [0, 0].as_slice()
        })
    }

    pub(crate) fn active_configuration_value(&self) -> u8 {
        todo!()
    }

    pub(crate) fn set_configuration(&self, _configuration: u8) -> Result<(), Error> {
        todo!()
    }

    pub(crate) fn reset(&self) -> Result<(), Error> {
        todo!()
    }

    /// SAFETY: `data` must be valid for `len` bytes to read or write, depending on `Direction`
    unsafe fn control_blocking(
        &self,
        _direction: Direction,
        _control: Control,
        _data: *mut u8,
        _len: usize,
        _timeout: Duration,
    ) -> Result<usize, TransferError> {
        todo!()
    }

    pub fn control_in_blocking(
        &self,
        _control: Control,
        _data: &mut [u8],
        _timeout: Duration,
    ) -> Result<usize, TransferError> {
        todo!()
    }

    pub fn control_out_blocking(
        &self,
        _control: Control,
        _data: &[u8],
        _timeout: Duration,
    ) -> Result<usize, TransferError> {
        todo!()
    }

    pub(crate) fn make_control_transfer(self: &Arc<Self>) -> TransferHandle<super::TransferData> {
        todo!()
    }

    pub(crate) fn claim_interface(
        self: &Arc<Self>,
        _interface: u8,
    ) -> Result<Arc<WebusbInterface>, Error> {
        todo!()
    }

    pub(crate) fn detach_and_claim_interface(
        self: &Arc<Self>,
        _interface: u8,
    ) -> Result<Arc<WebusbInterface>, Error> {
        todo!()
    }
}

pub(crate) struct WebusbInterface {
    pub interface_number: u8,
    pub(crate) device: Arc<WebusbDevice>,
}

impl WebusbInterface {
    pub(crate) fn make_transfer(
        self: &Arc<Self>,
        _endpoint: u8,
        _ep_type: EndpointType,
    ) -> TransferHandle<super::TransferData> {
        todo!()
    }

    pub fn control_in_blocking(
        &self,
        _control: Control,
        _data: &mut [u8],
        _timeout: Duration,
    ) -> Result<usize, TransferError> {
        todo!()
    }

    pub fn control_out_blocking(
        &self,
        _control: Control,
        _data: &[u8],
        _timeout: Duration,
    ) -> Result<usize, TransferError> {
        todo!()
    }

    pub fn set_alt_setting(&self, _alt_setting: u8) -> Result<(), Error> {
        todo!()
    }

    pub fn clear_halt(&self, _endpoint: u8) -> Result<(), Error> {
        todo!()
    }
}
