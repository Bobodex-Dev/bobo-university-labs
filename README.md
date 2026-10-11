# Bobo University labs

The programs and lab scripts behind the "Build the Bobodex" courses. Each
course is built here first; a chapter is written only from a tagged, verified
build of its lab.

- `TOOLCHAIN.md`: the exact versions every lab runs on, and the step 0 results.
- `fixtures/`: third-party programs the labs load, pinned by hash.
- `programs/`: the teaching copies ("Lite" programs).
- `courses/<course>/`: `NOTES.md` (the build journal) and one folder per chapter
  with `lab.toml`, `tests/`, `expected.json` and `script.sh`.
- `tools/verify-all`: run before building anything; nothing is built on a red tree.

Build and test from WSL (the Solana toolchain is installed there):

    tools/verify-all
