# IsItFurvesterAlready

This CLI solves one queston: Is it [Furvester](https://furvester.org/) already? Implemented in pure rust 🦀

`isitfurvester` calls the server https://isitfurvesteralready.info/ to get the current status.

## Building

You need to install rust and `cargo` first.

To build `isitfurvester` then clone this repo with 

```console
$ git clone https://github.com/maakinoh/isitfurvester.git
$ cd isitfurvester
```

Then run

```console
cargo build --release
```

## Usage

You can get the answer by simply calling

```console
$ isitfurvester
```

### Change the URL

You can change the URL of the server to query by setting the argument `--url` like

```console
$ isitfurvester --url="https://my.custom.domain"
```

### Get the remaining days

To just get the remaining days set the `--days` flag.

Like:
```console
$ isitfurvester --days
```

## Why?

I don't even know but it is in rust - must be good then xP
