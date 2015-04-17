/*! Scalar multiplication

# Representation of group elements
The correspondence between strings and group elements depends on the primitive
implemented by `scalarmult()`. The correspondence is not necessarily
injective in either direction, but it is compatible with scalar multiplication
in the group. The correspondence does not necessarily include all group
elements, but it does include all strings; i.e., every string represents at
least one group element.

# Representation of integers
The correspondence between strings and integers also depends on the primitive
implemented by `scalarmult()`. Every string represents at least one integer.

# Security model
`scalarmult` is designed to be strong as a component of various well-known
"hashed Diffie–Hellman" applications. In particular, it is designed to make the
"computational Diffie–Hellman" problem (CDH) difficult with respect to the
standard base.

`scalarmult` is also designed to make CDH difficult with respect to other
nontrivial bases. In particular, if a represented group element has small
order, then it is annihilated by all represented scalars. This feature allows
protocols to avoid validating membership in the subgroup generated by the
standard base.

NaCl does not make any promises regarding the "decisional Diffie–Hellman"
problem (DDH), the "static Diffie–Hellman" problem (SDH), etc. Users are
responsible for hashing group elements.

# Selected primitive
`scalarmult` is the function `crypto_scalarmult_curve25519` specified in
[Cryptography in NaCl](http://nacl.cr.yp.to/valid.html), Sections 2, 3, and 4.
This function is conjectured to be strong. For background see Bernstein,
"Curve25519: new Diffie-Hellman speed records," Lecture Notes in Computer
Science 3958 (2006), 207–228, http://cr.yp.to/papers.html#curve25519.
*/

pub use self::curve25519::*;
#[path="scalarmult/curve25519.rs"]
pub mod curve25519;
