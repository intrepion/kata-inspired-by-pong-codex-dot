# SDL3 macOS pilot source inventory

The macOS source-build packager requires `SOURCE-MANIFEST.json` at the Git
worktree root. Generate it from a normal checkout using local Git:

```sh
node experiments/sdl3/packaging/generate-source-manifest.cjs > SOURCE-MANIFEST.json
node experiments/sdl3/packaging/build-and-package-macos.cjs /path/to/new-output
```

Preserve or move an existing source manifest before redirecting output. If the
working tree contains intentionally modified or new **SDL3 packaging files**,
pass one `--allow-change RELATIVE_PATH` for each such file. For example:

```sh
node experiments/sdl3/packaging/generate-source-manifest.cjs \
  --allow-change experiments/sdl3/packaging/build-and-package-macos.cjs \
  --allow-change experiments/sdl3/packaging/pilot-resources.cjs \
  > SOURCE-MANIFEST.json
```

Every allowed path must actually differ from local HEAD. The generator compares
the selected worktree files with `git ls-tree HEAD`, fails for missing tracked
files, unexpected additions or changes, and symlinks, and ignores Cargo
`target` directories. It inventories SDL3, its shared crates and scenarios,
packaging sources and the project license. The packager regenerates and compares
the inventory before and after building, then embeds it and its hash with the
notice and build provenance in both the app and archive.

The manifest identifies the **local** HEAD plus explicit local packaging
changes. Local Git and a self-generated inventory do not authenticate that HEAD
is the intended remote commit: independently compare the commit and relevant
tree with the reviewed repository. The inventory does not cover unselected
repository paths, Cargo configuration, toolchains, environment variables,
downloaded dependency contents or the final linked-binary attribution. It does
not establish release or license clearance. Windows/Linux behavior has not been
verified for this Mac packager.
