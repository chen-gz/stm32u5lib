use crate::usb::bus::Bus;
use crate::usb::controlpipe::ControlPipe;
use crate::usb::endpoint::Endpoint;
use crate::usb::{ep_fifo_size, Config, Dir, EndpointData, In, OtgInstance, Out};
use core::marker::PhantomData;
use embassy_usb_driver::{Direction, EndpointAddress, EndpointAllocError, EndpointInfo, EndpointType};

/// USB OTG driver.
pub struct Driver<'d, const MAX_EP_COUNT: usize> {
    config: Config,
    ep_in: [Option<EndpointData>; MAX_EP_COUNT],
    ep_out: [Option<EndpointData>; MAX_EP_COUNT],
    ep_out_buffer: &'d mut [u8],
    ep_out_buffer_offset: usize,
    instance: OtgInstance<'d, MAX_EP_COUNT>,
}

impl<'d, const MAX_EP_COUNT: usize> Driver<'d, MAX_EP_COUNT> {
    /// Initializes the USB OTG peripheral.
    ///
    /// # Arguments
    ///
    /// * `ep_out_buffer` - An internal buffer used to temporarily store received packets.
    /// Must be large enough to fit all OUT endpoint max packet sizes.
    /// Endpoint allocation will fail if it is too small.
    /// * `instance` - The USB OTG peripheral instance and its configuration.
    /// * `config` - The USB driver configuration.
    pub fn new(ep_out_buffer: &'d mut [u8], instance: OtgInstance<'d, MAX_EP_COUNT>, config: Config) -> Self {
        Self {
            config,
            ep_in: [None; MAX_EP_COUNT],
            ep_out: [None; MAX_EP_COUNT],
            ep_out_buffer,
            ep_out_buffer_offset: 0,
            instance,
        }
    }

    /// Returns the total amount of words (u32) allocated in dedicated FIFO.
    fn allocated_fifo_words(&self) -> u16 {
        self.instance.extra_rx_fifo_words + ep_fifo_size(&self.ep_out) + ep_fifo_size(&self.ep_in)
    }

    /// Creates an [`Endpoint`] with the given parameters.
    fn alloc_endpoint<D: Dir>(
        &mut self,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Endpoint<'d, D>, EndpointAllocError> {
        trace!(
            "allocating type={:?} mps={:?} interval_ms={}, dir={:?}",
            ep_type,
            max_packet_size,
            interval_ms,
            D::dir()
        );

        if D::dir() == Direction::Out {
            if self.ep_out_buffer_offset + max_packet_size as usize > self.ep_out_buffer.len() {
                error!("Not enough endpoint out buffer capacity");
                return Err(EndpointAllocError);
            }
        };

        let fifo_size_words = match D::dir() {
            Direction::Out => (max_packet_size + 3) / 4,
            // INEPTXFD requires minimum size of 16 words
            Direction::In => u16::max((max_packet_size + 3) / 4, 16),
        };

        if fifo_size_words + self.allocated_fifo_words() > self.instance.fifo_depth_words {
            error!("Not enough FIFO capacity");
            return Err(EndpointAllocError);
        }

        let eps = match D::dir() {
            Direction::Out => &mut self.ep_out[..self.instance.endpoint_count],
            Direction::In => &mut self.ep_in[..self.instance.endpoint_count],
        };

        // Find free endpoint slot
        let slot = eps.iter_mut().enumerate().find(|(i, ep)| {
            if *i == 0 && ep_type != EndpointType::Control {
                // reserved for control pipe
                false
            } else {
                ep.is_none()
            }
        });

        let index = match slot {
            Some((index, ep)) => {
                *ep = Some(EndpointData {
                    ep_type,
                    max_packet_size,
                    fifo_size_words,
                });
                index
            }
            None => {
                error!("No free endpoints available");
                return Err(EndpointAllocError);
            }
        };

        trace!("  index={}", index);

        let state = &self.instance.state.ep_states[index];
        if D::dir() == Direction::Out {
            // Buffer capacity check was done above, now allocation cannot fail
            unsafe {
                *state.out_buffer.get() = self.ep_out_buffer.as_mut_ptr().offset(self.ep_out_buffer_offset as _);
            }
            self.ep_out_buffer_offset += max_packet_size as usize;
        }

        Ok(Endpoint {
            _phantom: PhantomData,
            regs: self.instance.regs,
            state,
            info: EndpointInfo {
                addr: EndpointAddress::from_parts(index, D::dir()),
                ep_type,
                max_packet_size,
                interval_ms,
            },
        })
    }
}

impl<'d, const MAX_EP_COUNT: usize> embassy_usb_driver::Driver<'d> for Driver<'d, MAX_EP_COUNT> {
    type EndpointOut = Endpoint<'d, Out>;
    type EndpointIn = Endpoint<'d, In>;
    type ControlPipe = ControlPipe<'d>;
    type Bus = Bus<'d, MAX_EP_COUNT>;

    fn alloc_endpoint_in(
        &mut self,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointIn, EndpointAllocError> {
        self.alloc_endpoint(ep_type, max_packet_size, interval_ms)
    }

    fn alloc_endpoint_out(
        &mut self,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointOut, EndpointAllocError> {
        self.alloc_endpoint(ep_type, max_packet_size, interval_ms)
    }

    fn start(mut self, control_max_packet_size: u16) -> (Self::Bus, Self::ControlPipe) {
        let ep_out = self
            .alloc_endpoint(EndpointType::Control, control_max_packet_size, 0)
            .unwrap();
        let ep_in = self
            .alloc_endpoint(EndpointType::Control, control_max_packet_size, 0)
            .unwrap();
        assert_eq!(ep_out.info.addr.index(), 0);
        assert_eq!(ep_in.info.addr.index(), 0);

        trace!("start");

        let regs = self.instance.regs;
        let cp_setup_state = &self.instance.state.cp_state;
        (
            Bus {
                config: self.config,
                ep_in: self.ep_in,
                ep_out: self.ep_out,
                inited: false,
                instance: self.instance,
            },
            ControlPipe {
                max_packet_size: control_max_packet_size,
                setup_state: cp_setup_state,
                ep_out,
                ep_in,
                regs,
            },
        )
    }
}
