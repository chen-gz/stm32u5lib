3291

USB on-the-go full-speed (OTG_FS)

RM0456

register to determine the enumeration speed and perform the steps listed in Endpoint
initialization on enumeration completion on page 3264.
At this point, the device is ready to accept SOF packets and perform control transfers on
control endpoint 0.

72.16.4

Host programming model
Channel initialization
The application must initialize one or more channels before it can communicate with
connected devices. To initialize and enable a channel, the application must perform the
following steps:
1.

Program the OTG_GINTMSK register to unmask the following:

2.

Channel interrupt
–

Non-periodic transmit FIFO empty for OUT transactions (applicable when
operating in pipelined transaction-level with the packet count field programmed
with more than one).

–

Non-periodic transmit FIFO half-empty for OUT transactions (applicable when
operating in pipelined transaction-level with the packet count field programmed
with more than one).

3.

Program the OTG_HAINTMSK register to unmask the selected channels’ interrupts.

4.

Program the OTG_HCINTMSK register to unmask the transaction-related interrupts of
interest given in the host channel interrupt register.

5.

Program the selected channel’s OTG_HCTSIZx register with the total transfer size, in
bytes, and the expected number of packets, including short packets. The application
must program the PID field with the initial data PID (to be used on the first OUT
transaction or to be expected from the first IN transaction).

6.

Program the OTG_HCCHARx register of the selected channel with the device’s
endpoint characteristics, such as type, speed, direction, and so forth. (The channel can
be enabled by setting the channel enable bit to 1 only when the application is ready to
transmit or receive any packet).

Halting a channel
The application can disable any channel by programming the OTG_HCCHARx register with
the CHDIS and CHENA bits set to 1. This enables the OTG_FS host to flush the posted
requests (if any) and generates a channel halted interrupt. The application must wait for the
CHH interrupt in OTG_HCINTx before reallocating the channel for other transactions. The
OTG_FS host does not interrupt the transaction that has already been started on the USB.
Before disabling a channel, the application must ensure that there is at least one free space
available in the non-periodic request queue (when disabling a non-periodic channel) or the
periodic request queue (when disabling a periodic channel). The application can simply
flush the posted requests when the request queue is full (before disabling the channel), by
programming the OTG_HCCHARx register with the CHDIS bit set to 1 which automatically
clears the CHENA bit to 0.
The application is expected to disable a channel on any of the following conditions:

<!-- pagebreak -->

