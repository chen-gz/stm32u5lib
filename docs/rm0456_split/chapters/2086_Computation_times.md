2093

Public key accelerator (PKA)

53.5.2

RM0456

Computation times
The following tables summarize the PKA computation times, expressed in AHB clock
cycles.
Table 521. Modular exponentiation
Exponent length
(in bits)
3

17
216 + 1

1024

2048

3072

4096

Modulus length (in bits)
Mode
1024

2048

3072

4096

Normal

124600

491000

684000

1133200

Fast

22700

82000

178000

311000

Normal

135700

531400

772400

1288000

Fast

33800

122500

266500

465800

Normal

180000

693700

1126200

1907200

Fast

78200

284700

620400

1085000

Protected

9958000

-

-

-

Normal

5850000

-

-

-

Fast

5748000

-

-

-

(1)

CRT

1775000

-

-

-

Protected

-

63886000

-

-

Normal

-

42240000

-

-

Fast

-

41832000

-

-

(1)

CRT

-

11670000

-

-

Protected

-

-

199403000

-

Normal

-

-

136830000

-

Fast

-

-

136325000

-

(1)

CRT

-

-

36886000

-

Protected

-

-

-

454318000

Normal

-

-

-

316000000

Fast

-

-

-

315226000

(1)

-

-

-

84577000

CRT

1. CRT stands for chinese remainder theorem optimization (MODE bitfield= 0x07).

Table 522. ECC scalar multiplication(1)
Modulus length (in bits)
160

192

256

320

384

512

521

640

-

1590000

3083000

5339000

8518000

17818000

21053000

31826000

1. These times depend on the number of 1s included in the scalar parameter, and include the computation of
Montgomery parameter R2.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Public key accelerator (PKA)
Table 523. ECDSA signature average computation time(1) (2)
Modulus length (in bits)
160

192

256

320

384

512

521

640

-

1500000

2744000

4579000

7184000

14455000

16685000

24965000

1. These values are average execution times of random moduli of given length, as they depend upon the
length and the value of the modulus.
2. The execution time for the moduli that define the finite field of NIST elliptic curves is shorter than that
needed for the moduli used for Brainpool elliptic curves or for random moduli of the same size.

Table 524. ECDSA verification average computation times
Modulus length (in bits)
160

192

256

320

384

512

521

640

1011000

1495000

2938000

5014000

7979000

16804000

19254000

29582000

Table 525. ECC double base ladder average computation times
Modulus length (in bits)
160

192

256

320

384

512

521

640

967000

1419000

2768000

4784000

7547000

15854000

18257000

28257000

Table 526. ECC projective to affine average computation times
Modulus length (in bits)
160

192

256

320

384

512

640

47600

78000

148300

253000

419000

838400

1049300

Table 527. ECC complete addition average computation times
Modulus length (in bits)
160

192

256

320

384

512

640

10000

12000

18000

26000

39000

53000

89000

Table 528. Point on elliptic curve Fp check average computation times
Modulus length (in bits)
160

192

256

320

384

512

521

640

3400

4200

6100

8300

10900

17200

-

-

RM0456 Rev 6

<!-- pagebreak -->

