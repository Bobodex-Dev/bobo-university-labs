# Fixtures

Programs our labs call are loaded from these files, never from the network,
so results do not change when a program is upgraded on mainnet.

## mpl_core.so (Metaplex Core)

| Field | Value |
|---|---|
| Program id | `CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d` |
| Last deployed in slot | 444040451 |
| Size | 848,320 bytes |
| sha256 | in `mpl_core.so.sha256` |
| Dumped | 2026-10-06 |
| Command | `solana program dump -u m CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d fixtures/mpl_core.so` |

To refresh: dump again, update the hash file and this table, run
`tools/verify-all`, and record any change in behaviour in the affected
course's `NOTES.md`.
