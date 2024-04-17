- Imports: num-bigint for big integers, num_traits for traits like One and Zero, and rand for random number generation.
- Generate Parameters: use a known large prime p and base g = 2. The prime is a safe prime.
- Key Generation:
  Alice and Bob both generate their private keys (a and b) randomly such that they are less than p.
  They compute their public keys (a_pub and b_pub) using modular exponentiation.
- Compute Shared Secret: Both compute the shared secret and check if they match

```
cargo run
```

```
Shared secret is: 62605773398404882222287129859063440242479678334062504723084552518681823160915929299012959776369640440735685197580617473360871155618672762371179310016888820706988801320359382207920187517217056272461901840956219663903320802684987542944261344819446176830534896378656250028119493951043923004467232099303013050187
```

### updrading naive version:

1. Use a Cryptographically Secure Prime
   Instead of using a fixed prime or generating it inadequately, ensure the prime is cryptographically secure. For Diffie-Hellman, this typically means using a "safe prime" (where (p-1)/2 is also prime).

2. Validate Public Keys
   Before computing the shared secret, validate the received public keys to prevent certain types of attacks (e.g., small subgroup attacks). Ensure that: The received public key is not 0 or 1 and is less than p.
