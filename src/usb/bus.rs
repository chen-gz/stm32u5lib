use usb_device::bus::PollResult;
use usb_device::endpoint::{EndpointAddress,  EndpointType};
use usb_device::{Result, UsbDirection};

// Use async_trait to allow async fns in traits, but only mark
// as async the functions that need to be.
// #[async_trait::async_trait]
pub trait USBBus {
    /// Allocate a new endpoint.
    /// This is an instantaneous operation.
    fn alloc_ep(
        &mut self,
        dir: UsbDirection,
        addr: EndpointAddress,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> Result<EndpointAddress>;

    /// Enable the USB peripheral.
    /// This is a simple configuration step.
    fn enable(&mut self);

    /// Reset the USB peripheral state.
    fn reset(&self);

    /// Polling for USB events might take time if waiting for events.
    async fn poll(&self) -> PollResult;

    /// Write data to an endpoint.
    /// This operation could potentially block while waiting for the transmission to complete.
    async fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> Result<usize>;

    /// Read data from an endpoint.
    /// This function may need to wait until data is available.
    async fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> Result<usize>;

    /// Setting STALL state is typically immediate and may not require awaiting.
    fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool);

    /// Check if an endpoint is stalled.
    /// This is usually a quick register read.
    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool;

    /// Configuring remote wakeup is an instantaneous configuration operation.
    fn set_remote_wakeup_enabled(&self, enabled: bool);
}
