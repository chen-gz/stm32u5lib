3291

USB on-the-go full-speed (OTG_FS)

RM0456

72.16

OTG_FS programming model

72.16.1

Core initialization
The application must perform the core initialization sequence. If the cable is connected
during power-up, the current mode of operation bit in the OTG_GINTSTS (CMOD bit in
OTG_GINTSTS) reflects the mode. The OTG_FS controller enters host mode when an “A”
plug is connected or device mode when a “B” plug is connected.
This section explains the initialization of the OTG_FS controller after power-on. The
application must follow the initialization sequence irrespective of host or device mode
operation. All core global registers are initialized according to the core’s configuration:
1.

2.

3.

Program the following fields in the OTG_GAHBCFG register:
–

Global interrupt mask bit GINTMSK = 1

–

Rx FIFO non-empty (RXFLVL bit in OTG_GINTSTS)

–

Periodic Tx FIFO empty level

Program the following fields in the OTG_GUSBCFG register:
–

HNP capable bit

–

SRP capable bit

–

OTG_FS timeout calibration field

–

USB turnaround time field

The software must unmask the following bits in the OTG_GINTMSK register:
OTG interrupt mask
Mode mismatch interrupt mask

4.

<!-- pagebreak -->

The software can read the CMOD bit in OTG_GINTSTS to determine whether the
OTG_FS controller is operating in host or device mode.

RM0456 Rev 6

RM0456

72.16.2

USB on-the-go full-speed (OTG_FS)

Host initialization
To initialize the core as host, the application must perform the following steps:
1.

Program the HPRTINT in the OTG_GINTMSK register to unmask

2.

Program the OTG_HCFG register to select full-speed host

3.

Program the PPWR bit in OTG_HPRT to 1. This drives VBUS on the USB.

4.

Wait for the PCDET interrupt in OTG_HPRT0. This indicates that a device is
connecting to the port.

5.

Program the PRST bit in OTG_HPRT to 1. This starts the reset process.

6.

Wait at least 10 ms for the reset process to complete.

7.

Program the PRST bit in OTG_HPRT to 0.

8.

Wait for the PENCHNG interrupt in OTG_HPRT.

9.

Read the PSPD bit in OTG_HPRT to get the enumerated speed.

10. Program the HFIR register with a value corresponding to the selected PHY clock 1
11. Program the FSLSPCS field in the OTG_HCFG register following the speed of the
device detected in step 9. If FSLSPCS has been changed a port reset must be
performed.
12. Program the OTG_GRXFSIZ register to select the size of the receive FIFO.
13. Program the OTG_HNPTXFSIZ register to select the size and the start address of the
Non-periodic transmit FIFO for non-periodic transactions.
14. Program the OTG_HPTXFSIZ register to select the size and start address of the
periodic transmit FIFO for periodic transactions.
To communicate with devices, the system software must initialize and enable at least one
channel.

72.16.3

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

Clear the DCTL.SDIS bit. The core issues a connect after this bit is cleared.

3.

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

4.

Wait for the USBRST interrupt in OTG_GINTSTS. It indicates that a reset has been
detected on the USB that lasts for about 10 ms on receiving this interrupt.

5.

Wait for the ENUMDNE interrupt in OTG_GINTSTS. This interrupt indicates the end of
reset on the USB. On receiving this interrupt, the application must read the OTG_DSTS

RM0456 Rev 6

<!-- pagebreak -->

