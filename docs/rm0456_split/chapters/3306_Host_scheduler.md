RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
The channel-specific interrupt service routine for bulk and control OUT/SETUP
transactions is shown in the following code samples.
•

Interrupt service routine for bulk/control OUT/SETUP and bulk/control IN
transactions
a)

Bulk/control OUT/SETUP

Unmask (NAK/TXERR/STALL/XFRC)
if (XFRC)
{
Reset Error Count
Mask ACK
De-allocate Channel
}
else if (STALL)
{
Transfer Done = 1
Unmask CHH
Disable Channel
}
else if (NAK or TXERR )
{
Rewind Buffer Pointers
Unmask CHH
Disable Channel
if (TXERR)
{
Increment Error Count
Unmask ACK
}
else
{
Reset Error Count
}
}
else if (CHH)
{
Mask CHH
if (Transfer Done or (Error_count == 3))
{
De-allocate Channel
}
else
{
Re-initialize Channel
}
}

RM0456 Rev 6

<!-- pagebreak -->

