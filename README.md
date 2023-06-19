# lepton - a super simple multi-app Spin host

lapton is a demonstration project showing how the [Spin](https://developer.fermyon.com/spin/index) runtime can be hosted outside of the Spin command line.

## Background

It's tempting to identify Spin with the `spin` command line.  But the `spin` command line is a local developer environment.  The lifecycle of a production application is more like:

* author the application and test locally using `spin up`
* publish the application to a registry using `spin registry publish`
* run the application from the registry on a production service

lepton is certainly not a production service, but it demonstrates the skeleton of what a host might look like that is focused on _serving_ applications rather than _developing_ them.

* lepton does not accept `spin.toml` files: it operates only on registry references.
* lepton hosts multiple Spin applications in a single process.

lepton comes in two flavours, `lepton` and `tauon`.  `lepton` keeps the configuration as simple as possible: it loads and runs a defined set of applications, so that you can concentrate on how the hosting works.  `tauon` provides for dynamically starting and stopping applications, as would be needed in a real service.  (Although it is still not a real service.)

## Kicking the tyres

### Prerequisites

To build lepton, you will need a recent version of Rust.  I have tried it on Linux and Mac with Rust 1.69.

To build your own applications, you will need [Spin](https://developer.fermyon.com/spin/install).  I used Spin 1.3 to build the example applications.

### Applications

Both lepton flavours come with example configurations that refer to public registry packages.  So you can run them out of the box: that is, in both cases you should be able to `cargo run --release` them and see results.

If you would like to try with your own Spin applications, you must publish those applications.  If you publish the packages as private (which is the default for the GitHub registry), you will need to run `spin registry login` before running lepton; lepton does not come with its own login command.  But this is the only way in which lepton depends on the Spin CLI!

> This is because I am lazy, and the goal is to demonstrate hosting rather than registry auth.

## `lepton` aka 'smol lepton'

`lepton` runs the set of Spin applications defined in the `lepton.json` file.  It loads the application list at startup; to change the set of applications, you'll need to restart `lepton`.

### Running `lepton`

To try it out with the default apps:

```
cargo run --release
```

Please be patient - it may take a while to download the packages first time and it doesn't display progress messages! (Sorry. It's a demo.) It will display the Spin "available routes" message as each app loads.

To use it with your own apps:

* Create and build one or more Spin apps, and push them to a registry using `spin registry push`
* Create a `lepton.json` file in the same format as the sample one
* `cargo run --release -- <YOUR_LEPTON_FILE>`

> Rmember to `spin registry login` if your packages are private.

### How does it work?

* On startup, it reads the `lepton.json` file
* It uses the `spin_oci` crate to download the registry references
* For each application:
  * It creates a gadget called a `LockedApp`, which basically resolves a bunch of content-addressed references to usable files
  * It instantiates a HTTP trigger for that `LockedApp`
  * It runs that trigger - effectively a HTTP server - in a Tokio task

## `tauon` aka 'big lepton'

`tauon` adds a control plane, which allows you to start, stop and reconfigure applications while the server is running.  Some picky pedants will say it monitors a directory for configuration changes, but it's my repo and I can call the file system a control plane if I want to.

In `tauon`, each application is represented by its _own_ JSON file.  Dropping a JSON file into the `tauon` starts the app described in that file.  Deleting a JSON file from the `tauon` directory stops the app described in that file.  Editing a JSON file stops an app, then restarts it with a new configuration.

> Stop then restart isn't really what you want from a production service.  A production service would drain traffic from the old application and shift it over to the new one.  `tauon` isn't a production service.

### Running `tauon`

To try it out with the default apps:

```
cargo run --release --bin tauon
```

Initially wth the demo configuration you will see it one application.  To change the set of apps being served, delete files out of the `tauon` directory and/or copy files from the `tauon-tests` directory.  Beware, the file-to-app matching and error handling are very crude so things like copies of a file on the same port will likely not go great. (Sorry. It's a demo.)

To use it with your own apps:

* Create and build one or more Spin apps, and push them to a registry using `spin registry push`
* Create `*.json` files in the same format as the sample ones
* `cargo run --release` (or `cargo run --release -- <DIR_TO_WATCH>`)
* Copy your files into the `tauon` directory (or `DIR_TO_WATCH` if you overrode it)

> Rmember to `spin registry login` if your packages are private.

### How does it work?

Almost exactly the same as `lepton` - the big differences are down to the monitoring code, not the hosting code.

The main interesting difference is that `tauon` needs to be able to stop applications, so it cares about `RunningApp` instances more for their abort handles than their join handles.
