<img src="https://avatars.githubusercontent.com/u/81774317?s=200&v=4" width="40" />

# Autha

> Autha, pronounced `Otter` 🦦, is the service that manages user accounts and the associated delegation. ☄️

Autha is an OAuth2 server designed with **Rust** to allow extreme low resource usage, low _latency_ and high request throughput.<br />
It implements an account creation, connection and authorization delegation system.

**Status**:

[![Autha/bazel](https://github.com/Gravitalia/Autha/actions/workflows/ci.yml/badge.svg?branch=v3&event=push)](https://github.com/Gravitalia/Autha/actions/workflows/ci.yml) [![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FGravitalia%2FAutha.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2FGravitalia%2FAutha?ref=badge_shield)

## Security

> We want to guarantee our users **increased security**. This way, all users can see how we store data and also help us to improve the cryptographic systems.

- `Argon2`, `ChaCha20Poly1305`, `SHA-3` and `FPE` (with `AES`) are the hashing and cryptographic systems used
- Short expiration time (< `15 min.`)
- JWT with asymmetric key
- One-Time Usage OAuth token

#### Argon2

[Argon2id](https://en.wikipedia.org/wiki/Argon2) is a key-derivative hash function which resists to side-channel attacks and is more reliable against GPU cracking attacks.<br />
It allows us to manage the amount of memory used, the degree of parallelism as well as the number of iterations to do.

#### ChaCha20Poly1305

[ChaCha20](https://en.wikipedia.org/wiki/Salsa20) is an encryption function built around a pseudorandom function.<br />
[Poly1305](https://en.wikipedia.org/wiki/Poly1305) (MAC) allows to verify the integrity of the data as well as their veracity (authenticity).<br />
[ChaCha20Poly1305](https://en.wikipedia.org/wiki/ChaCha20-Poly1305) is an AEAD algorithm standardized by RFC. It allows to verify authenticity and confidentiality.

#### SHA-3

[SHA-3](https://en.wikipedia.org/wiki/SHA-3) is a hash function and is the latest version of the Secure Hashed Algorithm. Even if SHA-2 is not replaced by this version, SHA-3 can resist a length extension attack.

#### FPE & AES

[Format-preserving encryption](https://en.wikipedia.org/wiki/Format-preserving_encryption) aka FPE is an encryption function that provides the same output for the same text and the same format.<br />
To achieve encryption, we use [AES](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard), which is used by the US government.

## Database

> We try to use high-performance databases to optimize critical response times.

- Apache Cassandra
- Memcached

We are also working on implementing a possibility to use `PostgreSQL` instead of `Apache Cassandra`.

#### Casssandra

Apache [Cassandra](https://en.wikipedia.org/wiki/Apache_Cassandra) is a fast, fault-tolerant and ultra-scalable distributed database optimized for mass writing.<br />
We use Cassandra to manage user accounts and security; every _significant_ action taken by the user is logged.

#### Memcached

[Memcached](https://en.wikipedia.org/wiki/Memcached) is a key-value database with in-memory capability, extremely fast.
Used to cache public user data and redistribute it quickly as well as to unclog requests to Cassandra.

## License

[Mozilla Public License](https://github.com/Gravitalia/Autha/blob/master/LICENSE)
