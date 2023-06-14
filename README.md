# lepton - a super simple multi-app Spin host

## `lepton` aka 'smol lepton'

To try out:

```
cargo run --release
```

Please patient - it may take a while to download the packages first time and it doesn't display progress messages! (Sorry. It's a demo.) It will display the Spin "available routes" message as each app loads.

To use it with your own apps:

* Create and build one or more Spin apps, and push them to a registry using `spin registry push`
* Create a `lepton.json` file in the same format as the sample one
* `cargo run --release -- <YOUR_LEPTON_FILE>`

NOTE: lepton relies on Spin for registry login.  If your registry packages are private, you must have run `spin registry login` before running lepton.  (Sorry. It's a demo.)

## `tauon` aka 'big lepton'

To try out:

```
cargo run --release --bin tauon
```

To change the set of apps being served, delete files out of the `tauon` directory and/or copy files from the `tauon-tests` directory.  Dropping a file into `tauon` starts the app specified in the file; deleting a file out of `tauon` stops the app from that file.  The file-to-app matching and error handling are very crude so things like copies of a file on the same port will likely not go great. (Sorry. It's a demo.)

To use it with your own apps:

* Create and build one or more Spin apps, and push them to a registry using `spin registry push`
* Create `*.json` files in the same format as the sample ones
* `cargo run --release` (or `cargo run --release -- <DIR_TO_WATCH>`)
* Copy your files into the `tauon` directory (or `DIR_TO_WATCH` if you overrode it)

NOTE: tauon relies on Spin for registry login.  If your registry packages are private, you must have run `spin registry login` before running tauon.  (Sorry. It's a demo.)
