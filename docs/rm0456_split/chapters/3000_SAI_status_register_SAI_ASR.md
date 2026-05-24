RM0456 Rev 6

RM0456

Universal serial bus full-speed host/device interface (USB)
The mechanism is based on a table very similar to that for Device mode. See Table 738 to
understand the relationship between the DTOG bit buffers and the buffer usage.
Table 738. Isochronous memory buffers usage
Endpoint
Type

DTOG bit
value

Packet buffer used by the
USB peripheral

Packet buffer used by the
application software

0

USB_CHEP_TXRXBD_0
(ADDRn_TX / COUNTn_TX)
Buffer description table locations.

USB_CHEP_RXTXBD_0
(ADDRn_TX / COUNTn_TX)
Buffer description table locations

1

USB_CHEP_RXTXBD_0
(ADDRn_TX / COUNTn_TX)
Buffer description table locations

USB_CHEP_TXRXBD_0
(ADDRn_TX / COUNTn_TX)
Buffer description table locations.

0

USB_CHEP_RXTXBD_0
(ADDRn_RX / COUNTn_RX)
Buffer description table locations.

USB_CHEP_TXRXBD_0
(ADDRn_RX / COUNTn_RX)
Buffer description table locations.

1

USB_CHEP_TXRXBD_0
(ADDRn_RX / COUNTn_RX)
Buffer description table locations

USB_CHEP_RXTXBD_0
(ADDRn_RX / COUNTn_RX)
Buffer description table locations.

Transmit
(OUT)

Receive
(IN)

The isochronous behavior for an endpoint is selected by setting the UTYPE bits at 10 in its
USB_CHEPnR register; since there is no handshake phase the only legal values for the
STATRX/STATTX bit pairs are 00 (DISABLED) and 11 (VALID),
Just as in Device mode, the mechanism allows automatic toggle of the DTOG bit. Note that
in Host mode, at the same time as this toggle, the STATTX or STATRX of the completed
buffer is automatically set to DISABLED, permitting the future buffer to be accessed before
re-enabling it by setting it to 11 (VALID).

71.5.7

Suspend/resume events
The USB standard defines a special peripheral state, called SUSPEND, in which the
average current drawn from the USB bus must not be greater than 2.5 mA. This
requirement is of fundamental importance for bus-powered devices, while self-powered
devices are not required to comply to this strict power consumption constraint. In suspend
mode, the host PC sends the notification by not sending any traffic on the USB bus for more
than 3 ms: since a SOF packet must be sent every 1 ms during normal operations, the USB
peripheral detects the lack of 3 consecutive SOF packets as a suspend request from the
host PC and set the SUSP bit to 1 in USB_ISTR register, causing an interrupt if enabled.
Once the device is suspended, its normal operation can be restored by a so called
RESUME sequence, which can be started from the host PC or directly from the peripheral
itself, but it is always terminated by the host PC. The suspended USB peripheral must be
anyway able to detect a RESET sequence, reacting to this event as a normal USB reset
event.
The actual procedure used to suspend the USB peripheral is device dependent since
according to the device composition, different actions may be required to reduce the total
consumption.

RM0456 Rev 6

<!-- pagebreak -->

3134

Universal serial bus full-speed host/device interface (USB)

RM0456

A brief description of a typical suspend procedure is provided below, focused on the USBrelated aspects of the application software routine responding to the SUSP notification of
the USB peripheral:
1.

Set the SUSPEN bit in the USB_CNTR register to 1. This action activates the suspend
mode within the USB peripheral. As soon as the suspend mode is activated, the check
on SOF reception is disabled to avoid any further SUSP interrupts being issued while
the USB is suspended.

2.

Remove or reduce any static power consumption in blocks different from the USB
peripheral.

3.

Set SUSPRDY bit in USB_CNTR register to 1 to remove static power consumption in
the analog USB transceivers but keeping them able to detect resume activity.

4.

Optionally turn off external oscillator and device PLL to stop any activity inside the
device.

When an USB event occurs while the device is in SUSPEND mode, the RESUME
procedure must be invoked to restore nominal clocks and regain normal USB behavior.
Particular care must be taken to ensure that this process does not take more than 10 ms
when the wakening event is an USB reset sequence (see “Universal Serial Bus
Specification” for more details). The start of a resume or reset sequence, while the USB
peripheral is suspended, clears the SUSPRDY bit in USB_CNTR register asynchronously.
Even if this event can trigger a WKUP interrupt if enabled, the use of an interrupt response
routine must be carefully evaluated because of the long latency due to system clock restart;
to have the shorter latency before re-activating the nominal clock it is suggested to put the
resume procedure just after the end of the suspend one, so its code is immediately
executed as soon as the system clock restarts. To prevent ESD discharges or any other kind
of noise from waking-up the system (the exit from suspend mode is an asynchronous
event), a suitable analog filter on data line status is activated during suspend; the filter width
is about 70 ns.
The following is a list of actions a resume procedure must address:

<!-- pagebreak -->

