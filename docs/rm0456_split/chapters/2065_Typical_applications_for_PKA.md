RM0456 Rev 6

RM0456

53.3.5

Public key accelerator (PKA)

Typical applications for PKA
Introduction
The PKA can be used to accelerate a number of public key cryptographic functions. In
particular:
•

RSA encryption and decryption

•

RSA key finalization

•

CRT-RSA decryption

•

DSA and ECDSA signature generation and verification

•

DH and ECDH key agreement

Specifications of the above functions are given in following publications:
•

FIPS PUB 186-4, Digital Signature Standard (DSS), July 2013 by NIST

•

PKCS #1, RSA Cryptography Standard, v1.5, v2.1 and v2.2. by RSA Laboratories

•

IEEE1363-2000, IEEE Standard Specifications for Public-Key Cryptography, January
2000

•

ANSI X9.62-2005, Public Key Cryptography for the Financial Services Industry, The
Elliptic Curve Digital Signature Algorithm (ECDSA), November 2005

The principles of the main functions are described in this section, for a more detailed
description refer to the above cited documents.

RSA key pair
For the following RSA operations a public key and a private key information are defined as
below:
•

Alice transmits her public key (n, e) to Bob. Numbers n and e are very large positive
integers.

•

Alice keeps secret her private key d, also a very large positive integer. Alternatively this
private key can also be represented by a quintuple (p, q, dp, dq, qInv).

For more information on the above representations refer to the RSA specification.

RSA encryption/decryption principle
As recommended by the PKCS#1 specification, Bob, to encrypt message M using Alice’s
public key (n, e) must go through the following steps:
1.

Compute the encoded message EM = ENCODE(M), where ENCODE is an encoding
method.

2.

Turn EM into an integer m, with 0 ≤ m < n and (m, n) being coprimes.

3.

Compute ciphertext c = me mod n.

4.

Convert the integer c into a string ciphertext C.

RM0456 Rev 6

<!-- pagebreak -->

2093

Public key accelerator (PKA)

RM0456

Alice, to decrypt ciphertext c using her private key d, follows the steps indicated below:
1.

Convert the ciphertext C to an integer ciphertext representative c.

2.

If necessary, retrieve the prime factors (p, q) using (n, e, d) information, then compute
phi = (p - 1) * (q - 1). Refer to NIST SP800-56B Appendix C for details.

3.

Recover plaintext m = cd mod n = (me)d mod n. If the private key is the quintuple (p, q,
dp, dq, qInv), then plaintext m is obtained by performing the operations:
a)

m1 = cdp mod p

b)

m2 = cdq mod q

c)

h = qInv (m1 – m2) mod p

d)

m = m2 + h q

4.

Convert the integer message representative m to an encoded message EM.

5.

Recover message M = DECODE(EM), where DECODE is a decoding method.

Above operations can be accelerated by PKA using Modular exponentiation Ae mod n if the
private key is d, or RSA CRT exponentiation if the private key is the quintuple (p, q, dp, dq,
qInv).
Note:

The decoding operation and the conversion operations between message and integers are
specified in PKCS#1 standard.
For the decryption process protected version of modular exponentiation (MODE = 0x3) is
strongly recommended for security reason. For encryption process MODE = 0x3 cannot be
used, as it requires the knowledge of the private key.

Elliptic curve selection
For following ECC operations curve parameters are defined as below:
•

Curve corresponds to the elliptic curve field agreed among actors (Alice and Bob).
Supported curves parameters are summarized in Section 53.5.1: Supported elliptic
curves.

•

G is the chosen elliptic curve base point (also known as generator), with a large prime
order n (that is, n x G = identity element O).

ECDSA message signature generation
ECDSA (elliptic curve digital signature algorithm) signature generation function principle is
the following: Alice, to sign a message m using her private key integer dA, goes through the
following steps.

<!-- pagebreak -->

