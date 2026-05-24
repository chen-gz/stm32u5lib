RM0456 Rev 6

RM0456

Digital camera interface (DCMI)
detect a frame/line start or frame/line end. This means that there can be different codes for
the frame/line start and frame/line end with the unmasked bit position remaining the same.

Example
FS = 0xA5
Unmask code for FS = 0x10
In this case the frame start code is embedded in the bit 4 of the frame start code.

41.3.7

DCMI capture modes
This interface supports two types of capture: snapshot (single frame) and continuous grab.

Snapshot mode (single frame)
In this mode, a single frame is captured (CM = 1 of the DCMI_CR register). After the
CAPTURE bit is set in DCMI_CR, the interface waits for the detection of a start of frame
before sampling the data. The camera interface is automatically disabled (CAPTURE bit
cleared in DCMI_CR) after receiving the first complete frame. An interrupt is generated
(IT_FRAME) if it is enabled.
In case of an overrun, the frame is lost and the CAPTURE bit is cleared.
Figure 395. Frame capture waveforms in snapshot mode

DCMI_HSYNC

DCMI_VSYNC

Frame 1 captured

Frame 2
not captured

ai15832b

1. Here, the active state of DCMI_HSYNC and DCMI_VSYNC is 1.
2. DCMI_HSYNC and DCMI_VSYNC can change states at the same time.

Continuous grab mode
In this mode (CM bit = 0 in DCMI_CR), once the CAPTURE bit has been set in DCMI_CR,
the grabbing process starts on the next DCMI_VSYNC or embedded frame start depending
on the mode. The process continues until the CAPTURE bit is cleared in DCMI_CR. Once
the CAPTURE bit has been cleared, the grabbing process continues until the end of the
current frame.

RM0456 Rev 6

<!-- pagebreak -->

