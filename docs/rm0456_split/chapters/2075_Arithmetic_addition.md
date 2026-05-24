RM0456 Rev 6

RM0456

Public key accelerator (PKA)
Table 504. Modular reduction
Parameters with direction

IN

OUT

53.4.9

Value (note)

Storage

Size

PKA_CR

6 bits

MODE

0x0D

Operand length

(in bits, not null)

RAM@0x400

Modulus length

(in bits, 8 < value < 4160)

RAM@0x408

Operand A

(0 ≤ A < 2n < 24160)

RAM@0xA50
4160

Modulus value n

(odd integer only, n < 2

Result A mod n

(0 < result < n)

)

RAM@0xC68

64 bits

ROS

RAM@0xE78

Arithmetic addition
Arithmetic addition operation consists in the computation of A + B. Operation instructions
are summarized in Table 505.
Table 505. Arithmetic addition
Parameters with direction

IN

OUT

53.4.10

Value (note)

MODE

0x09

Operand length M

(in bits, not null)

Operand A

(0 ≤ A < 2

Storage

Size

PKA_CR

6 bits

RAM@0x408

64 bits

M)

RAM@0xA50

M)

RAM@0xC68

Operand B

(0 ≤ B < 2

Result: A+B

(0 ≤ result < 2M+1)

RAM@0xE78

ROS
ROS + 1

Arithmetic subtraction
Arithmetic subtraction operation consists in the following computations:
•

If A ≥ B result equals A - B

•

If A < B and M/32 residue is > 0 result equals A + 2int(M/32)*32+1 - B

•

If A < B and M/32 residue is 0 result equals A + 2int(M/32)*32 - B

For the last two bullets the 32-bit word following the most significant word of the output
equals 0xFFFF FFFF, as result is negative.
Operation instructions are summarized in Table 506.
Table 506. Arithmetic subtraction
Parameters with direction

IN

MODE

0x0A

Operand length M

(in bits, not null)

Operand A
Operand B

OUT

Value (note)

Result: A-B

(0 ≤ < A < 2

Storage

Size

PKA_CR

6 bits

RAM@0x408

64 bits

M)

RAM@0xA50

M)

RAM@0xC68

(0 ≤ < B < 2

(0 ≤ result < 2

M)

RM0456 Rev 6

ROS

RAM@0xE78

<!-- pagebreak -->

2093

Public key accelerator (PKA)

53.4.11

RM0456

Arithmetic multiplication
Arithmetic multiplication operation consists in the computation of AxB. Operation
instructions are summarized in Table 507.
Table 507. Arithmetic multiplication
Parameters with direction

IN

OUT

53.4.12

Value (note)

MODE

0x0B

Operand length M

(in bits, not null)

Storage

Size

PKA_CR

6 bits

RAM@0x408

64 bits

Operand A

M

(0 ≤ A < 2 )

RAM@0xA50

Operand B

M

(0 ≤ B < 2 )

RAM@0xC68

Result: AxB

(0 ≤ result < 22M)

RAM@0xE78

ROS
2xROS

Arithmetic comparison
Arithmetic comparison operation consists in the following computation:
•

If A = B then result = 0xED2C

•

If A > B then result = 0x7AF8

•

If A < B then result = 0x916A

Operation instructions for arithmetic comparison are summarized in Table 508.
Table 508. Arithmetic comparison
Parameters with direction

IN

OUT

53.4.13

Value (note)

MODE

0x0C

Operand length M

(in bits, not null)

Storage

Size

PKA_CR

6 bits

RAM@0x408

64 bits

Operand A

(0 ≤ A < 2

M)

RAM@0xA50

Operand B

(0 ≤ B < 2M)

RAM@0xC68

Result A?B

0xED2C, 0x7AF8 or 0x916A

RAM@0xE78

ROS
64 bits

RSA CRT exponentiation
For efficiency many popular crypto libraries such as OpenSSL RSA use the following
optimization for decryption and signing based on the chinese remainder theorem (CRT):
•

p and q are precomputed primes, stored as part of the private key

•

dP= d mod (p -1)

•
•

<!-- pagebreak -->

dQ= d mod (q -1) and

qinv= q-1 mod p

RM0456 Rev 6

RM0456

Public key accelerator (PKA)
These values allow the recipient to compute the exponentiation m = Ad (mod pq) more
efficiently as follows:
•

m1 = AdP mod p

•

m2 = AdQ mod q

•

h = qinv (m1 - m2) mod p, with m1> m2

•

m = m2 + hq mod pq

Operation instructions for computing CRT exponentiation Ad mod pq are summarized in
Table 509.
Table 509. CRT exponentiation
Parameters with direction

Value (note)

Storage

Size

PKA_CR

6 bits
64 bits

IN

MODE

0x07

IN

Operand length

(in bits, not null)

RAM@0x408

Operand dP

(0 < dP

< 2M/2)

RAM@0x730

Operand dQ

(0 < dQ < 2M/2)

RAM@0xE78

Operand qinv

< 2M/2)

RAM@0x948

IN

(0 < p < 2M/2)

RAM@0xB60

Prime q(1)

(0 < q< 2M/2)

RAM@0x1088

Operand A

(0 < A< 2M)

RAM@0x12A0

(0 ≤ result < pq)

RAM@0x838

Prime p

IN
OUT

(0 < qinv

(1)

d mod pq

Result: A

ROS / 2

ROS

1. Must be different from 2.

53.4.14

Point on elliptic curve Fp check
This operation consists in checking whether a given point P (x, y) satisfies or not the curves
over prime fields equation y2 = (x3 + ax + b) mod p, where a and b are elements of the
curve.
Operation instructions for point on elliptic curve Fp check are summarized in Table 510.

RM0456 Rev 6

<!-- pagebreak -->

2093

Public key accelerator (PKA)

RM0456
Table 510. Point on elliptic curve Fp check

Parameters with direction

IN

Value (note)

53.4.15

Size

PKA_CR

6 bits

MODE

0x28

Modulus length

(in bits, not null,
8 < value < 640)

RAM@0x408

Curve coefficient a sign

0x0: positive
0x1: negative

RAM@0x410

Curve coefficient |a|

(absolute value, |a| < p)

RAM@0x418

Curve coefficient b

(|b| < p)

RAM@0x520

Curve modulus value p

(odd integer prime,
0 < p < 2640)

RAM@0x470

Point P coordinate x

(x < p)

RAM@0x578

Point P coordinate y

(y < p)

RAM@0x5D0

Montgomery parameter
R2 mod n

OUT

Storage

-

0xD60D: point on curve
0xA3B7: point not on curve
Result: point P on curve
0xF946: x or y coordinate is not
smaller than modulus p

64 bits

EOS

RAM@0x4C8

RAM@0x680

64 bits

ECC Fp scalar multiplication
This operation consists in the computation of k x P (xP, yP), where P is a point on a curve
over prime fields and “x” is the elliptic curve scalar point multiplication. Result of the
computation is a point that belongs to the same curve or a point at infinity.
Operation instructions for ECC Fp scalar multiplication are summarized in Table 511.

Note:

Once this operation is started PKA control register and PKA memory is no more available.
Access is restored once BUSY bit is set to 0 by the PKA.
When this operation completes with errors due to unexpected hardware events, a PKA
tamper event is triggered to TAMP peripheral, and access to PKA RAM becomes blocked
until erased by hardware. PKA tamper is also triggered when the programmed input point is
not found on the input ECC curve. PKA operation "Point on elliptic curve" can be used to
avoid this.
Table 511. ECC Fp scalar multiplication
Parameters with direction
IN

IN

<!-- pagebreak -->

Value (note)

Storage

Size

PKA_CR

6 bits

MODE

0x20

Curve prime order n length

(in bits, not null,)

RAM@0x400

Curve modulus p length

(in bits, not null,
8 < value < 640)

RAM@0x408

Curve coefficient a sign

0x0: positive
0x1: negative

RAM@0x410

RM0456 Rev 6

64 bits

RM0456

Public key accelerator (PKA)
Table 511. ECC Fp scalar multiplication (continued)
Parameters with direction

IN

OUT

Value (note)

Storage

Curve coefficient |a|

(absolute value, |a| < p)

RAM@0x418

Curve coefficient b

(positive integer)

RAM@0x520

Curve modulus value p

(odd integer prime, 0 < p < 2640

)

640

)

RAM@0x1088
RAM@0x12A0

Scalar multiplier k

(0 ≤ k < 2

Point P coordinate xP

(x < p)

RAM@0x578

Point P coordinate yP

(y < p)

RAM@0x470

Curve prime order n

(integer prime)

RAM@0xF88

Result: k x P coordinate x’

(result < p)

RAM@0x578

Result: k x P coordinate y’

(result < p)

RAM@0x5D0

No errors: 0xD60D
Errors: 0xCBC9

RAM@0x680

ERROR Error k x P

Size

EOS

64 bits

When performing this operation the following special cases must be noted:
•

For k = 0 this function returns a point at infinity (0, 0) if curve parameter b is nonzero,
(0, 1) otherwise. For k different from 0 it might happen that a point at infinity is returned.
When the application detects this behavior a new computation must be carried out.

•

For k < 0 (that is, a negative scalar multiplication is required) the multiplier absolute
value
k = |-k| must be provided to the PKA. After the computation completion, the formula
-P = (x, -y) can be used to compute the y coordinate of the effective final result (the x
coordinate remains the same).

Note:

The error code is not affected when PKA automatically clears the PKA RAM at the end of
this protected operation. When the error output equals 0xD60D, the result output is not
affected either.

53.4.16

ECDSA sign
ECDSA signing operation (outlined in Section 53.3.5) is summarized in Table 512 (input
parameters) and in Table 513 (output parameters).
The application has to check if the output error is equal to 0xD60D, if it is different a new k
must be generated and the ECDSA sign operation must be repeated.

Note:

Once this operation is started PKA control register and PKA memory is no more available.
Access is restored once BUSY bit is set to 0 by the PKA.
When this operation completes with errors due to unexpected hardware events a PKA
tamper event is triggered to TAMP peripheral, and access to PKA RAM becomes blocked
until erased by hardware. PKA tamper is also triggered when the programmed input point is
not found on the input ECC curve. PKA operation "Point on elliptic curve" can be used to
avoid this.

RM0456 Rev 6

<!-- pagebreak -->

2093

Public key accelerator (PKA)

RM0456
Table 512. ECDSA sign - Inputs

Parameters with direction

IN

Value (note)

Storage

Size

PKA_CR

6 bits

MODE

0x24

Curve prime order n
length (nlen)

(in bits, not null)

RAM@0x400

Curve modulus p length

(in bits, 8 < value < 640)

RAM@0x408

Curve coefficient a sign

0x0: positive
0x1: negative

RAM@0x410

Curve coefficient |a|

(absolute value, |a| < p)

RAM@0x418

Curve coefficient b

(positive integer)

RAM@0x520

Curve modulus value p

(odd integer prime,
0 < p < 2640)

RAM@0x1088

Integer k(1)

(0 ≤ k < 2640)

RAM@0x12A0

Curve base point G
coordinate x

(x < p)

RAM@0x578

Curve base point G
coordinate y

(y < p)

RAM@0x470

Hash of message z

(hash size equal to nlen)(2)

RAM@0xFE8

Private key d

(0 < d)

RAM@0xF28

Curve prime order n

(integer prime)

RAM@0xF88

64 bits

EOS

1. This integer is usually a cryptographically secure random number, but in some cases k can be
deterministically generated.
2. Padding with zeroes or hash truncation must be used to have the hash parameter size equal to the curve
prime order n length.

Table 513. ECDSA sign - Outputs
Parameters with direction
OUT

Value (note)

Signature part r

(0 < r < n)

RAM@0x730

Signature part s

(0 < s < n)

RAM@0x788

0xD60D: successful computation, no
error
ERROR Result of signature 0xCBC9: failed computation
0xA3B7: signature part r is equal to 0
0xF946: signature part s is equal to 0

Note:

Storage

RAM@0xFE0

Size
EOS

64 bits

The error code is not affected when PKA automatically clears the PKA RAM at the end of
this protected operation. When the error output equals 0xD60D, the result output is not
affected either.

Extended ECDSA support
PKA also supports extended ECDSA signature, for which the inputs and the outputs are the
same as ECDSA signature (Table 512 and Table 513, respectively), with the addition of the
coordinates of the point kG. This extra output is defined in Table 514.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Public key accelerator (PKA)
Table 514. Extended ECDSA sign - Extra outputs

OUT

53.4.17

Parameters with direction

Value (note)

Storage

Curve point kG coordinate x1

(0 ≤ x1 < p)

RAM@0x1400

Curve point kG coordinate y1

(0 ≤ y1 < p)

RAM@0x1458

Size
EOS

ECDSA verification
ECDSA verification operation (outlined in Section 53.3.5) is summarized in Table 515 (input
parameters) and Table 516 (output parameters).
The application has to check if the output error is equal to 0xD60D, if different the signature
is not verified.
Table 515. ECDSA verification - Inputs
Parameters with direction

IN

Value (note)

Storage

Size

PKA_CR

6 bits

MODE

0x26

Curve prime order n
length (nlen)

(in bits, not null)

RAM@0x408

Curve modulus p length

(in bits, not null,
8 < value < 640)

RAM@0x4C8

Curve coefficient a sign

0x0: positive
0x1: negative

RAM@0x468

Curve coefficient |a|

(absolute value, |a| < p)

RAM@0x470

Curve modulus value p

(odd integer prime,
0< p < 2640)

RAM@0x4D0

Curve base point G
coordinate x

(x < p)

RAM@0x678

Curve base point G
coordinate y

(y < p)

RAM@0x6D0

Public-key curve point Q
coordinate xQ

(xQ < p)

RAM@0x12F8

Public-key curve point Q
coordinate yQ

(yQ < p)

RAM@0x1350

Signature part r

(0 < r < n)

RAM@0x10E0

Signature part s

(0 < s < n)

RAM@0xC68

Hash of message z

(hash size equal to nlen)(1)

RAM@0x13A8

Curve prime order n

(integer prime)

RAM@0x1088

64 bits

EOS

1. Padding with zeroes or hash truncation must be used to have the hash parameter size equal to the curve
prime order n length.

RM0456 Rev 6

<!-- pagebreak -->

2093

Public key accelerator (PKA)

RM0456
Table 516. ECDSA verification - Outputs

Parameters with direction

OUT

Value (note)
– 0xD60D: valid signature
– 0xA3B7: invalid signature

Result: ECDSA verify

Computed signature part r – (0 < r < n)

53.4.18

Storage

Size

RAM@0x5D0

64 bits

RAM@0x578

EOS

ECC complete addition
ECC complete addition computes the addition of two given points on an elliptic curve.
Operation instructions are summarized in Table 517.

Note:

The two input points and the resulting point are represented in Jacobian coordinates
(X,Y,Z). To input a point in affine coordinates (x,y) conversion (X,Y,Z) = (x,y,1) can be used.
To convert resulting point to Jacobian coordinates conversion (x,y)= (X/Z2,Y/Z3) can be
used.
Table 517. ECC complete addition
Parameters with direction

Storage

Size

PKA_CR

6 bits

MODE

0x23

Curve modulus p length

(in bits, not null,
8 < value < 640)

RAM@0x408

Curve coefficient a sign

– 0x0: positive
0x1: negative

RAM@0x410

Curve modulus value p

(odd integer prime,
0 < p < 2640)

RAM@0x470

Curve coefficient |a|

(absolute value, |a| < p)

RAM@0x418

First point P coordinate X

(x < p)

RAM@0x628

First point P coordinate Y

(y < p)

RAM@0x680

First point P coordinate Z

(z < p)

RAM@0x6D8

Second point Q coordinate X

(x < p)

RAM@0x730

Second point Q coordinate Y

(y < p)

RAM@0x788

Second point Q coordinate Z

(z < p)

RAM@0x7E0

Result coordinate X

(x < p)

RAM@0xD60

OUT Result coordinate Y

(y < p)

RAM@0xDB8

Result coordinate Z

(z < p)

RAM@0xE10

IN

53.4.19

Value (note)

64 bits

EOS

ECC double base ladder
ECC double base ladder operation consists in the computation of k*P+m*Q, where (P,Q) are
two points on an elliptic curve and (k,m) are two scalars. Operation instructions are
summarized in Table 518.
If the resulting point is the point at infinity (error code 0xA3B7), resulting coordinate equals
(0, 0).

<!-- pagebreak -->

RM0456 Rev 6

RM0456
Note:

Public key accelerator (PKA)
The two input points are represented in Jacobian coordinates (X, Y , Z). To input a point in
affine coordinates (x, y) conversion (X, Y, Z) = (x, y, 1) can be used. The result is
represented in affine coordinates (x ,y)
Table 518. ECC double base ladder
Parameters with direction

IN

OUT

53.4.20

Value (note)

Storage

Size

PKA_CR

6 bits

MODE

0x27

Curve prime order n length

(in bits, not null)

RAM@0x400

Curve modulus p length

(in bits, not null,
8 < value < 640)

RAM@0x408

Curve coefficient a sign

– 0x0: positive
– 0x1: negative

RAM@0x410

Curve coefficient |a|

(absolute value, |a| < p)

RAM@0x418

Curve modulus value p

(odd integer prime,
0 < p < 2640)

RAM@0x470

Integer k

(0 < k < 2640)

RAM@0x520

Integer m

(0 < m < 2

640)

RAM@0x578

First point P coordinate X

(x < p)

RAM@0x628

First point P coordinate Y

(y < p)

RAM@0x680

First point P coordinate Z

(z < p)

RAM@0x6D8

Second point Q coordinate X

(x < p)

RAM@0x730

Second point Q coordinate Y

(y < p)

RAM@0x788

Second point Q coordinate Z

(z < p)

RAM@0x7E0

Result coordinate x

(x < p)

RAM@0x578

Result coordinate y

(y < p)

RAM@0x5D0

Error code

– Point not at infinity: 0xD60D
– Point at infinity: 0xA3B7

RAM@0x520

64 bits

EOS

EOS
64 bits

ECC projective to affine
ECC projective to affine operation computes the conversion between the representation of a
point P in homogeneous projective coordinates and the representation of the point P in
affine coordinates. Namely, if the point is represented by the triple (X, Y, Z), it computes the
affine coordinates (x, y) = (X/Z, Y/Z).
All the operations are performed modulo the modulus p of the curve, which the point
belongs to. If the resulting point is the point at infinity (error code 0xA3B7), resulting
coordinate equals (0,0).
Operation instructions are summarized in Table 519.

RM0456 Rev 6

<!-- pagebreak -->

