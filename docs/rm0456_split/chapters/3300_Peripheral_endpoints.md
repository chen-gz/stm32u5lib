RM0456 Rev 6

RM0456

73.15.3

USB on-the-go high-speed (OTG_HS)

Device initialization
The application must perform the following steps to initialize the core as a device on powerup or after a mode change from host to device.
1.

Program the following fields in the OTG_DCFG register:
–

Device speed

–

Non-zero-length status OUT handshake

–

Periodic Frame Interval

2.

Program the Device threshold control register. This is required only if you are using
DMA mode and you are planning to enable thresholding.

3.

Clear the DCTL.SDIS bit. The core issues a connect after this bit is cleared.

4.

Program the OTG_GINTMSK register to unmask the following interrupts:
–

USB reset

–

Enumeration done

–

Early suspend

–

USB suspend

–

SOF

5.

Wait for the USBRST interrupt in OTG_GINTSTS. It indicates that a reset has been
detected on the USB that lasts for about 10 ms on receiving this interrupt.

6.

Wait for the ENUMDNE interrupt in OTG_GINTSTS. This interrupt indicates the end of
reset on the USB. On receiving this interrupt, the application must read the OTG_DSTS
register to determine the enumeration speed and perform the steps listed in Endpoint
initialization on enumeration completion.

At this point, the device is ready to accept SOF packets and perform control transfers on
control endpoint 0.

73.15.4

DMA mode
The OTG host uses the AHB master interface to fetch the transmit packet data (AHB to
USB) and receive the data update (USB to AHB). The AHB master uses the programmed
DMA address (OTG_HCDMAx register in host mode and
OTG_DIEPDMAx/OTG_DOEPDMAx register in peripheral mode) to access the data
buffers.

73.15.5

Host programming model
Channel initialization
The application must initialize one or more channels before it can communicate with
connected devices. To initialize and enable a channel, the application must perform the
following steps:

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

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

7.

Program the selected channels in the OTG_HCSPLTx register(s) with the hub and port
addresses (split transactions only).

8.

Program the selected channels in the OTG_HCDMAx register(s) with the buffer start
address (DMA transactions only).

Halting a channel
The application can disable any channel by programming the OTG_HCCHARx register with
the CHDIS and CHENA bits set to 1. This enables the OTG_HS host to flush the posted
requests (if any) and generates a channel halted interrupt. The application must wait for the
CHH interrupt in OTG_HCINTx before reallocating the channel for other transactions. The
OTG_HS host does not interrupt the transaction that has already been started on the USB.
To disable a channel in DMA mode operation, the application does not need to check for
space in the request queue. The OTG_HS host checks for space to write the disable
request on the disabled channel’s turn during arbitration. Meanwhile, all posted requests are
dropped from the request queue when the CHDIS bit in OTG_HCCHARx is set to 1.
Before disabling a channel, the application must ensure that there is at least one free space
available in the non-periodic request queue (when disabling a non-periodic channel) or the
periodic request queue (when disabling a periodic channel). The application can simply
flush the posted requests when the request queue is full (before disabling the channel), by
programming the OTG_HCCHARx register with the CHDIS bit set to 1 which automatically
clears the CHENA bit to 0.
The application is expected to disable a channel on any of the following conditions:

<!-- pagebreak -->

