use std::task::Poll;

use crate::{hotplug::HotplugEvent, Error};

pub(crate) struct WebusbHotplugWatch {}

impl WebusbHotplugWatch {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {})
    }

    pub(crate) fn poll_next(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<HotplugEvent> {
        Poll::Pending
    }
}
