# lepton - a super simple multi-app Spin host

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
