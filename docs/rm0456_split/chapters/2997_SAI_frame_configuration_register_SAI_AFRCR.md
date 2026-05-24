3134

Universal serial bus full-speed host/device interface (USB)

RM0456

The application software is responsible for DTOG and SW_BUF bits initialization according
to the first buffer to be used; this has to be done considering the special toggle-only property
that these two bits have. The end of the first transaction occurring after having set
DBL_BUF, triggers the special flow control of double-buffered bulk endpoints, which is used
for all other transactions addressed to this endpoint until DBL_BUF remain set. At the end of
each transaction the VTRX or VTTX bit of the addressed endpoint USB_CHEPnR register is
set, depending on the enabled direction. At the same time, the affected DTOG bit in the
USB_CHEPnR register is hardware toggled making the USB peripheral buffer swapping
completely software independent. Unlike common transactions, and the first one after
DBL_BUF setting, STAT bit pair is not affected by the transaction termination and its value
remains 11 (VALID). However, as the token packet of a new transaction is received, the
actual endpoint status is masked as 10 (NAK) when a buffer conflict between the USB
peripheral and the application software is detected (this condition is identified by DTOG and
SW_BUF having the same value, see Table 735). The application software responds to the
CTR event notification by clearing the interrupt flag and starting any required handling of the
completed transaction. When the application packet buffer usage is over, the software
toggles the SW_BUF bit, writing 1 to it, to notify the USB peripheral about the availability of
that buffer. In this way, the number of NAKed transactions is limited only by the application
elaboration time of a transaction data: if the elaboration time is shorter than the time
required to complete a transaction on the USB bus, no re-transmissions due to flow control
takes place and the actual transfer rate is limited only by the host PC.
The application software can always override the special flow control implemented for
double-buffered bulk endpoints, writing an explicit status different from 11 (VALID) into the
STAT bit pair of the related USB_CHEPnR register. In this case, the USB peripheral always
uses the programmed endpoint status, regardless of the buffer usage condition.

71.5.4

Double buffered channels: usage in Host mode
In Host mode the underlying transmit and receive methods for double buffered channels are
the same as those described for Device mode.
Similar to the Device mode table, a new table below Table 736: Bulk double-buffering
memory buffers usage (Host mode) shows the programming settings for OUT and IN
tokens.

<!-- pagebreak -->

