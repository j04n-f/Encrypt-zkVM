# zkVM

Privacy is growing in demand and becoming a central pillar of every new technology. What a few years ago, it was just an optional requirement. Today, it is a must. ZK proofs and Full Homomorphic encryption are two technologies that are trying to revolutionize the user privacy field. We wanted to know more about the ZK proofs field and explore the idea of mixing it with the FHE technology. A spoiler, it's too complex.

## Problem

- Not enough data to train the AI models, which results in biased models.
- Data democratization. Bigger players have access to more data than the smaller players.
- User has to choose between exposing their data or being left aside from using AI.
- Use of unknown sources of data.
- Lack of data monetization systems.

## Goal

The goal is to create a Marketplace where AI Engineers and AI User needs are satisfied.

**AI Engineers**

AI Engineers can train their models using Users' encrypted data and receive compensation each time someone uses the model. Meanwhile, the User can track who uses its data and ask for a percentage of the AI Engineer benefits. Models training will use FHE encrypted data and compute a ZK proof to verify the computation correctness allowing the user to track their data usage.

**AI Users**

Users can run inferences using AI Engineers models and verify the correct computation without requiring a third trusted party.

## Solution

Create a Web3 Data and AI Marketplace using ZK proofs and FHE. This repository contains one of the project pieces. The zkVM implements the required operations to allow the computation of arithmetic operations using ciphertexts. Also, the zkVM computes a zk-STARK to prove the correct computation of the process.

*This is a final Master degree project, to keep it simple we used a basic implementation of the LWE method*

## Technical Design

### Inputs

- **Public Inputs**: An array of non-encrypted inputs. The program reads the public inputs using the *READ* operation.
- **Secret Inputs**: An array of encrypted inputs. The program reads the secret inputs using the *READ2* operation.

### Learning with Errors

To encrypt and perform encrypted operations the VM uses a lattice-based cryptographySecret method named Leraning with Errors.

Let's start with:

`A . s = t`

Where `A` is a public matrix, `s` is the secret key, and `t` is a public vector. Anyone can use the Gaussian elimination method to find the secret key value.

Let's see how we can improve the equation by adding an *error* vector, also known as *noise*:

`A . s + e = t`

Now it's harder to calculate the secret key having the `A` matrix and `t` vector.

#### Encryption
---

The output of the encryption process is a ciphertext containing two values `u`: cipher polynomial and `v`: vector of the polynomial.

`u = t . e1 + e2 + m`

`v = A . e1 + e3`

Where `m` is the secret message. `e1`, `e2`, and `e3` are random error values.

#### Decryption
---

Use the secret key:

`d = v — s . u`

Where `d` is the decrypted message. This equals to:

`d = t . e1 + e3 + m — A . s . e1 — s .e`

Now add and subtract `e.e1` on the right side:

`d = e . e1 + e3 + m + e1 [ t — ( A . s + e ) ] — s . e2`

We can cancel `A . s + e = t` because we know it:

`d = m + e . e1 + e3 — s . e2`

Errors are very small noise. We can remove them by rounding them off using a particular mathematical method:

`d = m`

### Operations

The VM has the following operations:

| Operation | Definition                                                          | Value  | Shift   |
| --------- | ------------------------------------------------------------------- | ------ | ------- |
| PUSH      | Push a value to the top of the stack                                | 10_000 | Right 1 |
| READ      | Read a value from public inputs and push it to the top of the stack | 10_001 | Right 1 |
| READ2     | Read a value from secret inputs and push it to the top of the stack | 10_010 | Right 5 |
| ADD       | Add two elements from the top of the stack                          | 01_000 | Left 1  |
| ADD2      | Add two ciphertexts from the top of the stack                       | 01_011 | Left 5  |
| SADD      | Add an element and a ciphertext from the top of the stack           | 01_010 | Left 1  |
| MUL       | Multiply two elements from the top of the stack                     | 01_001 | Left 1  |
| SMUL      | Multiply an element and a ciphertext from the top of the stack      | 01_100 | Left 1  |

### State Machines

#### System
---

The System counts the clock steps.

| Clock |
| ----- |
| 0     |
| 1     |
| 2     |
| 3     |

#### Stack
---

The Stack executes the VM operations. The trace table contains the stack depth and the registries. For example, an ADD operation:

*Program*:

```
Push.1 Push.2 Add
```

*Trace*:

| Stack Depth | R0 | R0 | R0 | R0 |
| ----------- | -- | -- | -- | -- |
| 0           | 0  | 0  | 0  | 0  |
| 1           | 1  | 0  | 0  | 0  |
| 2           | 2  | 1  | 0  | 0  |
| 1           | 3  | 0  | 0  | 0  |

#### Decoder
---

The Decoder translates the Operation Code to its bits representation. For example, an ADD operation:

*Program*:

```
Push.1 Push.2 Add
```

*Trace*:

| B0 | B1 | B1 | B3 | B4 |
| -- | -- | -- | -- | -- |
| 0  | 0  | 0  | 0  | 1  |
| 0  | 0  | 0  | 0  | 1  |
| 0  | 0  | 0  | 1  | 0  |
| 0  | 0  | 0  | 0  | 0  |

#### Chiplets
---

The Chiplets apply a Rescue-Prime round to the Operation Code and Value. The trace contains the bits representation of the operation and each round values:

| B0 | H0 | H2 | H3 | H4 |
| -- | -- | -- | -- | -- |
| 1  | 5  | 6  | 2  | 5  |
| 1  | 5  | 7  | 8  | 9  |
| 1  | 5  | 7  | 0  | 0  |
| 0  | 5  | 7  | 0  | 0  |

### Program Hash

To Program Hash Program uses the [Rescue-Prime Paper](https://eprint.iacr.org/2020/1143.pdf).

The Rescue-Prime sponge function is a cryptographic algorithm designed to transform input data into a fixed-size output, which is useful in hashing and encryption. It operates in two main stages: absorbing and squeezing. During the absorbing phase, it takes in data (known as the input message) and combines it with an internal state by applying specific mathematical transformations. In the squeezing phase, it generates a fixed-length output by further processing this internal state. Rescue-Prime is designed for an efficient arithmetization in zero-knowledge proofs.

**Rescue-Prime Hash Parameters**

- *State Length*: 4
- *Number of Rounds*: 14
- *Cycle Length*: 16

The Rescue-Prime implementation absorbs the elements between permutations instead of absorbing them before applying them because it's easier to arithmetize.

### Arithmetization

To compute the ZK proof the VM uses [Winterfell](https://github.com/facebook/winterfell).

#### Assertions
---

The AIR assertions are the following:

- Clock value at 0 equals 0
- Stack Depth value at 0 equals 0
- Stack Registries at 0 equal 0
- Stack Registries at length() - 1 equal to Outputs
- Hash values at 0 equal 0
- Hash values at length() - 1 equal to Program Hash

#### Transitions
---

**Flags**

Flags enable or disable an operation. For example, an ADD operation:

`(1 - b0) * b1 * (1 - b2) * (1 - b3) * (1 - b4)`

VM uses *b0* and *b1* to represent **Shr** or **Shl** operations.

**Constrtains**

All constraints must return 0 for a valid transition. Each constraint has an associated degree. The degree depends on the degree of the polynomial (number of multiplied columns).

*Clock Increment*

The execution of an Operation increases the Clock by 1

`clk' - (clk + 1) = 0 || degree 1`

*Shr or Shl*

Operations must Shl, Shr or don't shift the stack.

`b0 * b1 = 0 || degree 2`

*Multiplication*

The next top of the stack value equals to the multiplication of the two previous top of the stack values.

`s0' - (s0 * s1) = 0 || degree 2`

*Rescue-Prime Hash*

The Program Hash uses periodic constraints. Periodic constraints ensure that certain values or conditions repeat over a predefined cycle. The Hash flag and ARK values are cyclic values that repeat over a cycle depending on the round step.

## Example

The following [example](examples/linear_regression/src/main.rs) represents a linear regression to compute an insurance price:

```text
# Compute
# b0 + (b1 * x1) + (b2 * x2) + (b3 * x3) + (b4 * x4)

read2
read
smul # (b1 * x1)
read2
read
smul # (b2 * x2)
add2 # (b1 * x1) + (b2 * x2)
read2
read
smul # (b3 * x3)
add2 # (b1 * x1) + (b2 * x2) + (b3 * x3)
read2
read
smul # (b4 * x4)
add2 # (b1 * x1) + (b2 * x2) + (b3 * x3) + (b4 * x4)
read
sadd # (b1 * x1) + (b2 * x2) + (b3 * x3) + (b4 * x4) + b0
```

Client generates the secret key, secret inputs and public inputs:

```rust
let b0 = 1u8;
let b1 = 3u8;
let b2 = 2u8;
let b3 = 4u8;
let b4 = 2u8;

let clear_x1 = 2u8;
let clear_x2 = 3u8;
let clear_x3 = 3u8;
let clear_x4 = 2u8;

let plaintext_modulus: u32 = 8u32;
let ciphertext_modulus: u32 = 128u32;
let k: usize = 4;
let std = 2.412_390_240_121_573e-5;

let parameters = LweParameters::new(plaintext_modulus, ciphertext_modulus, k, std);
let client_key = ServerKey::new(parameters);

let x1 = client_key.encrypt(clear_x1);
let x2 = client_key.encrypt(clear_x2);
let x3 = client_key.encrypt(clear_x3);
let x4 = client_key.encrypt(clear_x4);

InputData::new(&[b1, b2, b3, b4, b0], &[x1, x2, x3, x4], &client_key);
```

Server proves the VM execution:

```rust
let payload = InputData::read_from_bytes(&input_data).unwrap();

let path = Path::new("program.txt");

let program = Program::load(path).unwrap();

let inputs = ProgramInputs::new(payload.public_inputs(), payload.secret_inputs(), payload.server_key());

let (hash, output, proof) = vm::prove(program, inputs).unwrap();

OutputData::new(hash, proof, output);
```

Client verifies the VM execution:

```rust
let results = OutputData::read_from_bytes(&output_data).unwrap();

let result = FheUInt8::new(&results.output()[..5]);

let clear_result = client_key.decrypt(&result);

let min_opts = AcceptableOptions::MinConjecturedSecurity(95);

verify::<ProcessorAir, Blake3, DefaultRandomCoin<Blake3>>(
  results.proof().clone(),
    PublicInputs::new(results.hash().to_elements(), results.output(), client_key),
    &min_opts,
  ).unwrap()
```

## References

- [Anatomu of a STARK](https://aszepieniec.github.io/stark-anatomy/)
- [BrainSTARK](https://aszepieniec.github.io/stark-brainfuck/)
- [Winterfell](https://github.com/facebook/winterfell)
- [Distaff](https://github.com/GuildOfWeavers/distaff)
- [MidenVM](https://github.com/0xPolygonMiden/miden-vm)
