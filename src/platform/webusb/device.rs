use std::{sync::Arc, time::Duration};

use wasm_bindgen_futures::{js_sys::Array, wasm_bindgen::JsCast, JsFuture};
use web_sys::{js_sys::Uint8Array, UsbControlTransferParameters, UsbDevice, UsbInTransferResult};

use crate::{
    descriptors::{validate_config_descriptor, DESCRIPTOR_TYPE_CONFIGURATION},
    transfer::{Control, Direction, EndpointType, TransferError, TransferHandle},
    DeviceInfo, Error,
};

pub(crate) struct WebusbDevice {
    pub device: Arc<UsbDevice>,
    config_descriptors: Vec<Vec<u8>>,
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
            let device = Arc::new(device);
            if device.vendor_id() == d.vendor_id
                && device.product_id() == d.product_id
                && device.serial_number() == d.serial_number
            {
                JsFuture::from(device.open()).await.unwrap();

                let config_descriptors = extract_decriptors(&device).await;

                return Ok(Arc::new(Self {
                    device,
                    config_descriptors,
                }));
            }
        }
        Err(Error::other("device not found"))
    }

    // pub(crate) fn handle_events(&self) {
    //     todo!()
    // }

    pub(crate) fn configuration_descriptors(&self) -> impl Iterator<Item = &[u8]> {
        self.config_descriptors.iter().map(|d| &d[..])
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

    pub(crate) fn make_control_transfer(self: &Arc<Self>) -> TransferHandle<super::TransferData> {
        todo!()
    }

    pub(crate) async fn claim_interface(
        self: &Arc<Self>,
        interface_number: u8,
    ) -> Result<Arc<WebusbInterface>, Error> {
        JsFuture::from(self.device.claim_interface(interface_number))
            .await
            .unwrap();
        return Ok(Arc::new(WebusbInterface {
            interface_number,
            device: self.clone(),
        }));
    }

    pub(crate) fn detach_and_claim_interface(
        self: &Arc<Self>,
        _interface: u8,
    ) -> Result<Arc<WebusbInterface>, Error> {
        todo!()
    }
}

pub async fn extract_decriptors(device: &UsbDevice) -> Vec<Vec<u8>> {
    let num_configurations = device.configurations().length() as usize;
    let mut config_descriptors = Vec::with_capacity(num_configurations);

    let mut v = vec![0; 255];
    for i in 0..num_configurations {
        let setup = UsbControlTransferParameters::new(
            0,
            web_sys::UsbRecipient::Device,
            0x6, // Get descriptor: https://www.beyondlogic.org/usbnutshell/usb6.shtml#StandardDeviceRequests
            web_sys::UsbRequestType::Standard,
            ((DESCRIPTOR_TYPE_CONFIGURATION as u16) << 8) | (i as u16),
        );
        let res = JsFuture::from(device.control_transfer_in(&setup, 255))
            .await
            .unwrap();
        let res: UsbInTransferResult = JsCast::unchecked_from_js(res);
        let data = Uint8Array::new(&res.data().unwrap().buffer());
        data.copy_to(&mut v[..data.length() as usize]);
        config_descriptors.push(
            validate_config_descriptor(&v[..data.length() as usize])
                .map(|_| v.iter().copied().take(data.length() as usize).collect())
                .unwrap(),
        )
    }
    config_descriptors
}

pub async fn extract_string(device: &UsbDevice, id: u16) -> String {
    let mut v = vec![0; 255];
    let setup = UsbControlTransferParameters::new(
        0,
        web_sys::UsbRecipient::Device,
        0x6, // Get descriptor: https://www.beyondlogic.org/usbnutshell/usb6.shtml#StandardDeviceRequests
        web_sys::UsbRequestType::Standard,
        (0x03_u16 << 8) | (id),
    );
    let res = JsFuture::from(device.control_transfer_in(&setup, 255))
        .await
        .unwrap();
    let res: UsbInTransferResult = JsCast::unchecked_from_js(res);
    let data = Uint8Array::new(&res.data().unwrap().buffer());
    data.copy_to(&mut v[..data.length() as usize]);

    String::from_utf16(
        &v.drain(2..v[0] as usize)
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|c| ((c[1] as u16) << 8) | c[0] as u16)
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

pub(crate) struct WebusbInterface {
    pub interface_number: u8,
    pub(crate) device: Arc<WebusbDevice>,
}

impl WebusbInterface {
    pub(crate) fn make_transfer(
        self: &Arc<Self>,
        endpoint: u8,
        ep_type: EndpointType,
    ) -> TransferHandle<super::TransferData> {
        TransferHandle::new(super::TransferData::new(
            self.device.clone(),
            self.clone(),
            endpoint,
            ep_type,
        ))
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
