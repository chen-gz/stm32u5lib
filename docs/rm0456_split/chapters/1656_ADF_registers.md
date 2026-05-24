1674

Audio digital filter (ADF)

RM0456

So ANMIN = 148 / 2 = 74 LSB
In Figure 390, the trigger value (THRH in red) is fixed to 148 LSB. The input signal is at
- 75 dBFS during 512 samples, then its value goes to - 62 dB for 11000 samples, and
finally it is reduced to - 70 dBFS.
The blue curve shows the sound level estimation (SDLVL) versus time. The black curve
shows the ambient noise estimation versus time, increasing or decreasing
logarithmically. During the learning phase, it reaches the SDLVL value.
In this example ANSLP = 6, FRSIZE = 3 (64 samples), LFRNB = 0 (2 frames),
HGOVR = 0 (4 frames), SNTHR = 1 (6 dB) and ANMIN = 74.
Figure 391. SAD example working with SADMOD = 1x

40.8

ADF registers
All the ADF registers must be accessed either in word (32-bit) or half-word (16-bit) formats.

40.8.1

ADF global control register (ADF_GCR)
Address offset: 0x000
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TRGO
rw

Bits 31:1 Reserved, must be kept at reset value.
Bit 0 TRGO: Trigger output control
This bit is set by software and reset by hardware. It is used to start the acquisition of several
filters synchronously. It is also used to synchronize several ADF together by controlling the
adf_trgo signal.
0: Write 0 has no effect. Read 0 means that the trigger can be set again to 1.
1: Write 1 generates a positive pulse on the adf_trgo signal and triggers the acquisition on
enabled filter having their ACQMOD[2:0] = 01x and selecting TRGO as trigger. Read 1
means that the trigger pulse is still active.

<!-- pagebreak -->

