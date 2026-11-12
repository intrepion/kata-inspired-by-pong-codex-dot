# Fyrox generated-state cleanup safety

Fyrox ResourceRegistry 1.0.1 defaults to `data/resources.registry`. Its mutable registry guard saves on drop when changed; synchronous save creates the parent folder. This is engine-generated runtime state, not a package-manifest file. The inspected Linux pilot contains exactly two bytes, `{}`. A file's empty contents alone do not prove our ownership.

## Current policy

- Existing pilot residuals remain untouched. No pre-run ownership receipt was taken for them, so **preserve** is the only supported automatic outcome.
- A future bounded pilot should run from a newly created, dedicated directory, recording its identity and whether the registry existed *before* launching. Do not use a shared project working directory.
- `scripts/packaging/fyrox-state.mjs` provides read-only before/after inspection. It never deletes anything. A new exact `{}` registry can only become a **review-candidate**, not automatic authorization to remove it.
- Preexisting files, nonempty/oversized state, symlinks, changed run-directory identity and missing receipts are preserved. An existing `data` directory or unrelated neighbors must not be removed.
- Review requires the process to be stopped, exclusive run-directory ownership and unchanged content at action time. A receipt is not a security boundary or proof against concurrent writers. No recursive cleanup is permitted; do not turn this inspection result into unchecked deletion.
- Real installation/uninstallation should separate runtime data from executable payloads, document retention and offer an explicit user-data removal choice. Redirecting Fyrox IO or disabling registry writes would need separate runtime/resource-loading tests; this batch does neither.

## Verification

Seven focused tests cover new empty state, preexisting empty state, no receipt, populated state, unrelated files, mismatched directory identity, absent state, and symlink/oversize refusal. All thirteen packaging tests pass on Omarchy ARM64. Inspection of the actual retained Fyrox pilot returned `preserve` because it has no pre-run receipt; no files were changed or removed.

This closes the unsafe-cleanup question with a conservative tested inspection contract, **not** the clean-uninstall or release gate. The earlier residual remains available for review. Archives, timing evidence and runtime source are unchanged.
