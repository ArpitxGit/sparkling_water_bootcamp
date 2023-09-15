# Lambda's Sparkling Water Bootcamp - Repo for challenges and learning path

Public repository for exercises, challenges and all the needs of the Sparkling Water Bootcamp.

## Week 1 - Forging your tools: Finite Fields

This first week will be focused on the development of one of the building blocks of Cryptography: Finite Fields. 

### Recommended material:
- [An introduction to mathematical cryptography](https://books.google.com.ar/books/about/An_Introduction_to_Mathematical_Cryptogr.html?id=BHuTQgAACAAJ&source=kp_book_description&redir_esc=y) - Chapter 1.
- [Finite Fields](https://www.youtube.com/watch?v=MAhmV_omOwA&list=PLFX2cij7c2PynTNWDBzmzaD6ij170ILbQ&index=8)
- [Constructing finite fields](https://www.youtube.com/watch?v=JPiXFn9WA5Y&list=PLFX2cij7c2PynTNWDBzmzaD6ij170ILbQ&index=6)
- [Cyclic groups](https://www.youtube.com/watch?v=UIhhs38IAGM&list=PLFX2cij7c2PynTNWDBzmzaD6ij170ILbQ&index=3)
- [Summary on Montgomery arithmetic](https://eprint.iacr.org/2017/1057.pdf)
- [Mersenne primes](https://eprint.iacr.org/2023/824.pdf)

### Challenges:
- [Implement Montgomery backend for 32 bit fields](https://github.com/lambdaclass/lambdaworks/issues/538).
- [Implement efficient Mersenne prime backend](https://github.com/lambdaclass/lambdaworks/issues/540).
- [Implement efficient backend for pseudo-Mersenne primes](https://github.com/lambdaclass/lambdaworks/issues/393).
- Compare specific field implementations with ordinary Montgomery arithmetic.

### Cryptography content:
- [Serious Cryptography](https://books.google.com.ar/books/about/Serious_Cryptography.html?id=1D-QEAAAQBAJ&source=kp_book_description&redir_esc=y), Chapters 9 & 10.

### Exercises
- Implement naïve version of RSA.
- $7$ is a generator of the multiplicative group of $Z_p^\star$, where $p = 2^{64} - 2^{32} +1$. Find the generators for the $2^{32}$ roots of unity. Find generators for subgroups of order $2^{16} + 1$ and $257$.
- Define in your own words what is a group, a subgroup, a ring and a field.
- What are the applications of the Chinese Remainder Theorem in Cryptography?
- Find all the subgroups of the multiplicative group of $Z_{29}^\star$

## Supplementary Material
- [Polynomial Secret Sharing](https://decentralizedthoughts.github.io/2020-07-17-polynomial-secret-sharing-and-the-lagrange-basis/)
- [Polynomials over a Field](https://decentralizedthoughts.github.io/2020-07-17-the-marvels-of-polynomials-over-a-field/)
- [Fourier Transform](https://www.youtube.com/watch?v=spUNpyF58BY)
- [Fast Fourier Transform](https://www.youtube.com/watch?v=h7apO7q16V0)
