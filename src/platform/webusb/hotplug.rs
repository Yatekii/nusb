use std::{
    sync::{
        mpsc::{channel, Receiver, TryRecvError},
        Arc,
    },
    task::Poll,
};

use atomic_waker::AtomicWaker;
use wasm_bindgen_futures::spawn_local;
use web_sys::{
    wasm_bindgen::{prelude::Closure, JsCast},
    UsbConnectionEvent,
};

use crate::{hotplug::HotplugEvent, Error};

use super::{enumeration::device_to_info, DeviceId};

pub(crate) struct WebusbHotplugWatch {
    waker: Arc<AtomicWaker>,
    events: Receiver<HotplugEvent>,
}

impl WebusbHotplugWatch {
    pub fn new() -> Result<Self, Error> {
        let window = web_sys::window().unwrap();
        let navigator = window.navigator();
        let usb = navigator.usb();
        let waker = Arc::new(AtomicWaker::new());
        let (sender, receiver) = channel();
        {
            let sender = sender.clone();
            let waker = waker.clone();
            let onconnect = Closure::wrap(Box::new(move |event: UsbConnectionEvent| {
                let sender = sender.clone();
                let waker = waker.clone();
                spawn_local(async move {
                    let info = device_to_info(event.device()).await;
                    match info {
                        Ok(info) => {
                            let result = sender.clone().send(HotplugEvent::Connected(info));
                            if let Err(e) = result {
                                tracing::warn!(
                                    "Could not send the connect event to the internal channel: {e:?}",
                                )
                            } else {
                                waker.wake();
                            }
                        }
                        Err(e) => {
                            tracing::warn!(
                                 "Could not read device descriptors for internal connect event dispatch: {e:?}",
                             )
                        }
                    }
                })
            }) as Box<dyn FnMut(UsbConnectionEvent)>);
            usb.set_onconnect(Some((onconnect).as_ref().unchecked_ref()));
        }
        {
            let sender = sender.clone();
            let waker = waker.clone();
            usb.set_ondisconnect(Some(
                (Closure::wrap(Box::new(move |event: UsbConnectionEvent| {
                    let sender = sender.clone();
                    let waker = waker.clone();
                    let result = sender.send(HotplugEvent::Disconnected(crate::DeviceId(
                        DeviceId::from_device(&event.device()),
                    )));
                    if let Err(e) = result {
                        tracing::warn!(
                            "Could not send the disconnect event to the internal channel: {e:?}",
                        )
                    } else {
                        waker.wake();
                    }
                }) as Box<dyn FnMut(UsbConnectionEvent)>))
                .as_ref()
                .unchecked_ref(),
            ));
        }
        Ok(Self {
            waker,
            events: receiver,
        })
    }

    pub(crate) fn poll_next(&mut self, cx: &mut std::task::Context<'_>) -> Poll<HotplugEvent> {
        self.waker.register(cx.waker());
        match self.events.try_recv() {
            Ok(event) => Poll::Ready(event),
            Err(TryRecvError::Empty) => Poll::Pending,
            Err(TryRecvError::Disconnected) => Poll::Pending,
        }
    }
}
