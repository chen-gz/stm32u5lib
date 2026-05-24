RM0456 Rev 6

RM0456

Digital camera interface (DCMI)
If the DCMI_VSYNC signal goes active before the number of lines is specified in the
DCMI_CWSIZE register, then the capture stops and an IT_FRAME interrupt is generated
when enabled.
Figure 398. Data capture waveforms

DCMI_HSYNC

DCMI_VSYNC
HOFFCNT
CAPCNT

Data not captured in this phase

Data captured in this phase
MS35934V2

1. Here, the active state of DCMI_HSYNC and DCMI_VSYNC is 1.
2. DCMI_HSYNC and DCMI_VSYNC can change states at the same time.

41.3.9

DCMI JPEG format
To allow JPEG image reception, it is necessary to set the JPEG bit of the DCMI_CR register.
JPEG images are not stored as lines and frames, so the DCMI_VSYNC signal is used to
start the capture while DCMI_HSYNC serves as a data enable signal. The number of bytes
in a line may not be a multiple of 4. This case must be carefully handled since a DMA
request is generated each time a complete 32-bit word has been constructed from the
captured data. When an end of frame is detected and the 32-bit word to be transferred has
not been completely received, the remaining data are padded with zeros and a DMA
request is generated.
The crop feature and embedded synchronization codes cannot be used in JPEG format.

41.3.10

DCMI FIFO
A 8-word FIFO is implemented to manage data rate transfers on the AHB. The DCMI
features a simple FIFO controller with a read pointer incremented each time the camera
interface reads from the AHB, and a write pointer incremented each time the camera
interface writes to the FIFO. There is no overrun protection to prevent the data from being
overwritten if the AHB interface does not sustain the data transfer rate.
In case of overrun or errors in the synchronization signals, the FIFO is reset and the DCMI
interface waits for a new start of frame.

RM0456 Rev 6

<!-- pagebreak -->

