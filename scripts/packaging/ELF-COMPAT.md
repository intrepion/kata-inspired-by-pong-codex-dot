# ELF compatibility report

From the repository root, run `python3 scripts/packaging/elf_compat.py /absolute/path/to/binary --source-revision FULL_COMMIT_SHA > report.json`. The optional source revision is a caller-supplied claim and is explicitly marked unverified. No source checkout or commit is inferred from an ELF.

The helper opens the file read-only without following a final symlink and with nonblocking open before the regular-file check, hashes its bytes, invokes `/usr/bin/readelf` on an inherited Linux file descriptor, rehashes the same opened file and reports ELF class, byte order, machine, type, interpreter, `DT_NEEDED`, and required GLIBC, GLIBCXX and CXXABI version nodes. It forces the C locale, rejects readelf diagnostics, checks the version-needs record and library counts against the section and dynamic tags, and reads requirements only from `Version needs`, not definitions exported by a library. Unsupported or truncated formats fail rather than being treated as empty. A missing needs section is explicitly labeled as not proving compatibility. Numeric maxima indicate symbol-version requirements; special named ABI nodes are listed separately. The input binary is never executed.

This is static ELF evidence, not a guarantee of running on a distribution. It cannot identify dynamically loaded libraries, transitive system requirements, kernel/CPU/GPU/driver needs, physical input behavior, release readiness, or complete licensing and provenance. Empty version requirements need not mean broad compatibility. The helper supports Linux `/proc/self/fd` and GNU readelf; tests include synthetic readelf examples and an actual read-only ELF inspection.

Run the focused tests from the repository root:

```sh
python3 -m unittest discover -s scripts/packaging -p 'test_elf_compat.py'
```
