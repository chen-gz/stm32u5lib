RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
if (XFRC)
{
Reset Error Count
Mask ACK
De-allocate Channel
}
else
if (STALL or FRMOR)
{
Mask ACK
Unmask CHH
Disable Channel
if (STALL)
{
Transfer Done = 1
}
}
else
if (NAK or TXERR)
{
Rewind Buffer Pointers
Reset Error Count
Mask ACK
Unmask CHH
Disable Channel
}
else
if (CHH)
{
Mask CHH
if (Transfer Done or (Error_count == 3))
{
De-allocate Channel
}
else
{
Re-initialize Channel (in next b_interval - 1 Frame)
}
}
else
if (ACK)
{
Reset Error Count
Mask ACK
}

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

The application uses the NPTXFE interrupt in OTG_GINTSTS to find the
transmit FIFO space.
Interrupt IN
Unmask (NAK/TXERR/XFRC/BBERR/STALL/FRMOR/DTERR)
if (XFRC)
{
Reset Error Count
Mask ACK
if (OTG_HCTSIZx.PKTCNT == 0)
{
De-allocate Channel
}
else
{
Transfer Done = 1
Unmask CHH
Disable Channel
}
}
else
if (STALL or FRMOR or NAK or DTERR or BBERR)
{
Mask ACK
Unmask CHH
Disable Channel
if (STALL or BBERR)
{
Reset Error Count
Transfer Done = 1
}
else
if (!FRMOR)
{
Reset Error Count
}
}
else
if (TXERR)
{
Increment Error Count
Unmask ACK
Unmask CHH
Disable Channel
}
else

<!-- pagebreak -->

