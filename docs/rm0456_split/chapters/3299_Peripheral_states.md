3460

USB on-the-go high-speed (OTG_HS)

RM0456

This section explains the initialization of the OTG_HS controller after power-on. The
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

OTG_HS timeout calibration field

–

USB turnaround time field

The software must unmask the following bits in the OTG_GINTMSK register:
OTG interrupt mask
Mode mismatch interrupt mask

4.

73.15.2

The software can read the CMOD bit in OTG_GINTSTS to determine whether the
OTG_HS controller is operating in host or device mode.

Host initialization
To initialize the core as host, the application must perform the following steps:
1.

Program the HPRTINT in the OTG_GINTMSK register to unmask

2.

Program the OTG_HCFG register to select full-speed host

3.

Program the PPWR bit in OTG_HPRT to 1. The port is now considered powered
however this does not actually drive VBUS on the USB, so a further action must be
taken to control the external circuitry in the system so as to enable VBUS generation.

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

<!-- pagebreak -->

