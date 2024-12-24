use std::ffi::c_void;

use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{
    js_sys::{Object, Uint8Array},
    wasm_bindgen::JsCast,
    UsbInTransferResult, UsbOutTransferResult, UsbTransferStatus,
};

use crate::transfer::{
    notify_completion, Completion, ControlIn, ControlOut, EndpointType, PlatformSubmit,
    PlatformTransfer, RequestBuffer, ResponseBuffer, TransferInner,
};

pub struct TransferData {
    device: super::Device,
    interface: super::Interface,
    endpoint: u8,
    ep_type: EndpointType,
    written_bytes: usize,
    status: UsbTransferStatus,
    data: Vec<u8>,
}

impl TransferData {
    pub(crate) fn new(
        device: super::Device,
        interface: super::Interface,
        endpoint: u8,
        ep_type: EndpointType,
    ) -> Self {
        Self {
            device,
            interface,
            endpoint,
            ep_type,
            written_bytes: 0,
            status: UsbTransferStatus::Ok,
            data: vec![],
        }
    }
}

impl TransferData {
    fn store_and_notify(
        data: Vec<u8>,
        status: UsbTransferStatus,
        written_bytes: usize,
        user_data: *mut c_void,
    ) {
        unsafe {
            let transfer = user_data as *mut TransferInner<TransferData>;
            let t = (*transfer).platform_data();

            t.data = data;
            t.status = status;
            t.written_bytes = written_bytes;
            notify_completion::<TransferData>(user_data);
        }
    }
}

impl PlatformTransfer for TransferData {}

impl PlatformSubmit<Vec<u8>> for TransferData {
    unsafe fn submit(&mut self, data: Vec<u8>, user_data: *mut c_void) {
        let device = self.device.clone();
        let ep_type = self.ep_type;
        let endpoint_number = self.endpoint;
        spawn_local(async move {
            let (written_bytes, status) = match ep_type {
                EndpointType::Control => {
                    panic!("Control is unsupported for submit");
                }
                EndpointType::Isochronous => {
                    panic!("Isochronous is unsupported for submit");
                }
                EndpointType::Bulk => {
                    let array = Uint8Array::from(data.as_slice());
                    let array_obj = Object::try_from(&array).unwrap();

                    let result = JsFuture::from(
                        device
                            .device
                            .transfer_out_with_buffer_source(endpoint_number, array_obj)
                            .unwrap(),
                    )
                    .await
                    .unwrap();

                    let transfer_result: UsbOutTransferResult = JsCast::unchecked_from_js(result);
                    (
                        transfer_result.bytes_written() as usize,
                        transfer_result.status(),
                    )
                }
                EndpointType::Interrupt => {
                    panic!("Interrupt is unsupported for submit");
                }
            };

            Self::store_and_notify(data, status, written_bytes, user_data);
        });
    }

    unsafe fn take_completed(&mut self) -> Completion<ResponseBuffer> {
        let data = ResponseBuffer::from_vec(self.data.clone(), self.written_bytes);
        let status = self.status;
        tracing::info!("{status:?}");
        Completion {
            data,
            status: Ok(()),
        }
    }
}

impl PlatformSubmit<RequestBuffer> for TransferData {
    unsafe fn submit(&mut self, data: RequestBuffer, user_data: *mut c_void) {
        let device = self.device.clone();
        let ep_type = self.ep_type;
        let endpoint_number = self.endpoint & (!0x80);
        let (mut data, len) = data.into_vec();
        spawn_local(async move {
            let status = match ep_type {
                EndpointType::Control => {
                    panic!("Control is unsupported for submit");
                }
                EndpointType::Isochronous => {
                    panic!("Isochronous is unsupported for submit");
                }
                EndpointType::Bulk => {
                    tracing::info!("{len}");
                    let result =
                        JsFuture::from(device.device.transfer_in(endpoint_number, len as u32))
                            .await
                            .unwrap();

                    let transfer_result: UsbInTransferResult = JsCast::unchecked_from_js(result);
                    let received_data = Uint8Array::new(&transfer_result.data().unwrap().buffer());
                    data.resize(received_data.length() as usize, 0);
                    received_data.copy_to(&mut data[..received_data.length() as usize]);

                    tracing::info!("{:?}", transfer_result.status());

                    transfer_result.status()
                }
                EndpointType::Interrupt => {
                    panic!("Interrupt is unsupported for submit");
                }
            };

            Self::store_and_notify(data, status, len, user_data);
        });
    }

    unsafe fn take_completed(&mut self) -> Completion<Vec<u8>> {
        let data = self.data.clone();
        let status = self.status;
        tracing::info!("{status:?}");
        Completion {
            data,
            status: Ok(()),
        }
    }
}

impl PlatformSubmit<ControlIn> for TransferData {
    unsafe fn submit(&mut self, _data: ControlIn, _user_data: *mut c_void) {
        todo!()
    }

    unsafe fn take_completed(&mut self) -> Completion<Vec<u8>> {
        todo!()
    }
}

impl PlatformSubmit<ControlOut<'_>> for TransferData {
    unsafe fn submit(&mut self, _data: ControlOut, _user_data: *mut c_void) {
        todo!()
    }

    unsafe fn take_completed(&mut self) -> Completion<ResponseBuffer> {
        todo!()
    }
}
