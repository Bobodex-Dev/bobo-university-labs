# Toolchain

Every lab is built and verified on exactly these versions. `tools/verify-all`
refuses to run on anything else, so a lab never passes on one machine and
fails on another.

| Tool | Version | How to install |
|---|---|---|
| Rust (host) | 1.99.0 (pinned in `rust-toolchain.toml`) | `rustup` reads the pin |
| Agave CLI | 4.3.0 (platform-tools v1.57) | `agave-install init 4.3.0` |
| Anchor CLI | 1.2.1 | `avm install 1.2.1 && avm use 1.2.1` |
| anchor-lang | 1.2.1, `default-features = false`, `features = ["solana-v4"]` | Cargo |
| mpl-core | 0.12.1, default features | Cargo |
| LiteSVM | 0.17.0 | Cargo (dev) |
| Mollusk | 0.16.0 | Cargo (dev), not yet used |
| Surfpool | 1.6.0 | release binary from the txtx/surfpool GitHub release |
| Node | 24 LTS (24.21.0) | `nvm install 24` |

Checked current on 2026-10-06: every row is the newest stable release on
that date (Anchor 1.2.1 was released that day).

## Step 0 gate (passed 2026-10-06)

`tools/toolchain-probe` is a one-instruction program that reads a Metaplex
Core asset the way the courses teach it. It is the standing proof that the
pinned versions work together, and `verify-all` runs it on every change.

What it showed:

1. **It builds.** A host `cargo check` and `cargo build-sbf` both pass with
   anchor-lang 1.2.1 on `solana-v4` and mpl-core 0.12.1, on Agave 4.3.0.
2. **anchor-lang defaults to `solana-v3`.** Its only default feature is
   `solana-v3`, so the `solana-v4` build needs `default-features = false`.
3. **`#[account(owner = mpl_core::ID)]` compiles under `solana-v4`**, even
   though mpl-core 0.12.1 is built on solana-program 3 and so uses the
   older `Pubkey` type.
4. **The template's test crates are older than the plan.** `anchor init` on
   1.2.1 still writes LiteSVM 0.10 and solana-* 3.x crates. This repo uses
   LiteSVM 0.17 with `solana-instruction` 3.5, `solana-keypair` 3.1,
   `solana-signer` 3.0, `solana-message` 4.2, `solana-transaction` 4.1 and
   `solana-pubkey` 4. A program id crosses into tests as
   `Pubkey::new_from_array(program::ID.to_bytes())`.
5. **It runs.** In LiteSVM 0.17 against the Core fixture: reading a real
   asset costs 2,999 compute units; an account Core does not own is
   refused with `ConstraintOwner` (2004); a plain collection is refused by
   the first-byte check (6000).
6. **The first-byte check is required, not defensive.** `BaseAssetV1::from_bytes`
   is plain Borsh and accepts trailing bytes. A collection named `AB` with a
   32-character uri ending in nine zero bytes decodes as an asset: the name
   length lands in the update-authority tag (2 = `Collection`) and the
   trailing zeros read as an empty name, an empty uri and no `seq`. With the
   check removed the program accepts that collection and reports its update
   authority as the asset's owner; the test
   `refuses_a_crafted_collection_that_decodes_as_an_asset` fails exactly
   then.
7. **Upgrading Agave replaces the SBF toolchain.** The first `cargo build-sbf`
   on 4.3.0 uninstalled platform-tools 1.52; a program pinned to Agave 3.x
   downloads its own again if switched back.
