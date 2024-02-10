use wasm_bindgen_futures::{js_sys::Array, wasm_bindgen::JsCast, JsFuture};
use web_sys::{UsbAlternateInterface, UsbConfiguration, UsbDevice, UsbInterface};

use crate::{BusInfo, DeviceInfo, Error, InterfaceInfo};

pub async fn list_devices() -> Result<impl Iterator<Item = DeviceInfo>, Error> {
    tracing::debug!("enumerating");

    async fn inner() -> Result<Vec<DeviceInfo>, Error> {
        let window = web_sys::window().unwrap();
        let navigator = window.navigator();
        let usb = navigator.usb();
        let devices = JsFuture::from(usb.get_devices()).await.unwrap();

        let devices: Array = JsCast::unchecked_from_js(devices);

        let mut result = vec![];
        for device in devices {
            let device: UsbDevice = JsCast::unchecked_from_js(device);

            result.push(DeviceInfo {
                bus_id: "webusb".to_string(),
                device_address: 0,
                vendor_id: device.vendor_id(),
                product_id: device.product_id(),
                device_version: ((device.usb_version_major() as u16) << 8)
                    | device.usb_version_minor() as u16,
                class: device.device_class(),
                subclass: device.device_subclass(),
                protocol: device.device_protocol(),
                speed: None,
                manufacturer_string: device.manufacturer_name(),
                product_string: device.product_name(),
                serial_number: device.serial_number(),
                interfaces: device
                    .configurations()
                    .into_iter()
                    .flat_map(&mut |c| {
                        let configuration: UsbConfiguration = JsCast::unchecked_from_js(c);
                        configuration.interfaces().into_iter().map(|i| {
                            let interface: UsbInterface = JsCast::unchecked_from_js(i);
                            let alternate = interface.alternates().into_iter().next().unwrap();
                            let alternate: UsbAlternateInterface =
                                JsCast::unchecked_from_js(alternate);
                            InterfaceInfo {
                                interface_number: interface.interface_number(),
                                class: alternate.interface_class(),
                                subclass: alternate.interface_subclass(),
                                protocol: alternate.interface_protocol(),
                                interface_string: alternate.interface_name(),
                            }
                        })
                    })
                    .collect(),
                port_chain: vec![],
                max_packet_size_0: 255,
            });
        }

        Ok(result)
    }

    Ok(inner().await.unwrap().into_iter())
}

pub fn list_buses() -> Result<impl Iterator<Item = BusInfo>, Error> {
    Ok(vec![].into_iter())
}
