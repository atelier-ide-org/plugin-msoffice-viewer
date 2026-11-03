# Microsoft Office (`msoffice-viewer`)

A native plugin of [Atelier IDE](https://atelier-ide-org.github.io/docs/): Word, Excel and PowerPoint documents (`.docx`, `.xlsx`, `.pptx` and their variants) open in a read-only viewer, read without any office suite, the same way on every system, and export to PDF. See `doc/en-US.md` for what it does.

It was a built-in plugin of the IDE; this repository keeps its history.

## Install

In Atelier (0.3 or later), open **Plugins**, then **Available**, and click **Install**: it is installed once, for every project, and turned on. The releases are also in [atelier-ide-org/releases](https://github.com/atelier-ide-org/releases/releases) (`plugins/msoffice-viewer/vX.Y.Z`): download `msoffice-viewer.atelier-plugin` and, in **Plugins**, click **Install a plugin from a file** (the + button). The file holds the library of every system.

## How it works

The plugin is a dynamic library (`msoffice_viewer`) that talks to the IDE through a C interface only ([atelier-ide-org/api](https://github.com/atelier-ide-org/api)): it draws with its own egui, and the IDE paints what it gives back. So it builds alone, with any Rust compiler, and loads in every version of Atelier that reads the version of the interface its `plugin.json` gives (`abi`).

## Releases

Raise the version of `Cargo.toml` and push it to `main`: the **Release** workflow (`.github/workflows/release.yml`) builds the library for Windows, Linux and macOS (x86_64 and aarch64), then publishes `msoffice-viewer.atelier-plugin`, `plugin.json` and `msoffice-viewer-SHA256SUMS` as the release `plugins/msoffice-viewer/vX.Y.Z` of `atelier-ide-org/releases`, where the IDE finds it. Started by hand, it builds the version again and replaces its files.

It needs one secret of the organization: `ATELIER_RELEASES`, which writes the releases of `atelier-ide-org/releases`.

## Develop

```sh
git clone git@github.com:atelier-ide-org/plugin-msoffice-viewer.git
cd plugin-msoffice-viewer
cargo test
```

It is also the submodule `plugins/msoffice-viewer` of [atelier-ide-org/app](https://github.com/atelier-ide-org/app). To try it, make the file to install for this system (`target/<profile>/msoffice-viewer.atelier-plugin`), then install it in Atelier (Plugins, the + button):

```powershell
./package.ps1          # release
./package.ps1 -Dev     # debug
```

```sh
./package.sh           # release
./package.sh --debug   # debug
```
