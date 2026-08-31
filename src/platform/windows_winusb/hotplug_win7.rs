// Windows 7 does not have `CM_Register_Notification` and related APIs.

use std::task::{Context, Poll};

use crate::{hotplug::HotplugEvent, Error, ErrorKind};

pub(crate) struct WindowsHotplugWatch {}

impl WindowsHotplugWatch {
    pub fn new() -> Result<WindowsHotplugWatch, Error> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "hotplug is not supported on Windows 7",
        ))
    }

    pub fn poll_next(&mut self, _cx: &mut Context) -> Poll<HotplugEvent> {
        unimplemented!()
    }
}
