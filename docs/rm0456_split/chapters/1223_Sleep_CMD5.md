The DPSM does not send more data than indicated by DATALENGTH.

Send CMD12 by the CPSM.
CMD12 sets the card to Idle mode.

RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)
When reading data from the card the CMD12 end bit must be sent earliest at the same time
as the card read data block last data bit. This requires the CMD12 sending to be tied to the
data block reception timing. The following stop Open-ended Multiple block read data block
procedure applies:
1.

Before starting the data transfer, set DTMODE to “block data transfer ending with
STOP_TRANSMISSION command”.

2.

Wait for all data to be received by the DPSM and read from the FIFO (DATAEND flag).
–

3.

The DPSM does not receive more data than indicated by DATALENGTH, even if
the card is sending more data.

Send CMD12 with CMDSTOP bit set by the CPSM.
–

CMD12 stops the Card sending more data and set the card to Idle mode. Any
ongoing block transfer is aborted by the Card.

Note:

The SDMMC does not receive any more data from the card when DATACOUNT = 0, even
when the card continues sending data.

31.6.3

Sleep (CMD5)
The e•MMC card may be switched between a Sleep state and a Standby state by CMD5. In
the Sleep state the power consumption of the card is minimized and the Vcc power supply
may be switched off.
The CMD5 (SLEEP) is used to initiate the state transition from Standby state to Sleep state.
The card indicates Busy, pulling down SDMMC_D0, during the transition phase. The Sleep
state is reached when the card stops pulling down the SDMMC_DO line.
To set the card into Sleep state the following procedure applies:
1.

Enable interrupt on BUSYD0END.

2.

Send CMD5 (SLEEP).

3.

On BUSYD0END interrupt, card is in Sleep state.

4.

Vcc power supply can be switched off.

The CMD5 (AWAKE) is used to initiate the state transition from Sleep state to Standby state.
The card indicates Busy, pulling down SDMMC_D0, during the transition phase. The
Standby state is reached when the card stops pulling down the SDMMC_DO line.
To set the card into Sleep state the following procedure applies:
1.

Switch on Vcc power supply and wait unit minimum operating level is reached.

2.

Enable interrupt on BUSYD0END.

3.

Send CMD5 (AWAKE).

4.

On BUSYD0END interrupt card is in Standby state.

The Vcc power supply can be switched off only after the Sleep state has been reached. The
Vcc supply must be reinstalled before CMD5 (AWAKE) is sent.

RM0456 Rev 6

<!-- pagebreak -->

