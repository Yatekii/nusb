use std::ffi::c_void;

use crate::transfer::{
    Completion, ControlIn, ControlOut, PlatformSubmit, PlatformTransfer, RequestBuffer,
    ResponseBuffer,
};

pub struct TransferData {}

impl PlatformTransfer for TransferData {
    fn cancel(&self) {
        todo!()
    }
}

impl PlatformSubmit<Vec<u8>> for TransferData {
    unsafe fn submit(&mut self, _data: Vec<u8>, _user_data: *mut c_void) {
        todo!()
    }

    unsafe fn take_completed(&mut self) -> Completion<ResponseBuffer> {
        todo!()
    }
}

impl PlatformSubmit<RequestBuffer> for TransferData {
    unsafe fn submit(&mut self, _data: RequestBuffer, _user_data: *mut c_void) {
        todo!()
    }

    unsafe fn take_completed(&mut self) -> Completion<Vec<u8>> {
        todo!()
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
