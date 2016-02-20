[![Build Status](https://travis-ci.org/msgflo/msgflo-rust.svg?branch=master)](https://travis-ci.org/msgflo/msgflo-rust)
# msgflo-rust: Rust participant support for MsgFlo

[MsgFlo](https://github.com/msgflo/msgflo) is a distributed, polyglot FBP (flow-based-programming) runtime.
It integrates with other FBP tools like the [Flowhub](http://flowhub.io) visual programming IDE.
`msgflo-rust` makes it easy to create MsgFlo participants in Rust.

## Status

*Prototype*

* Can expose a Rust function on AMQP with a single in/outport
* Sends the MsgFlo participant discovery message on startup

See below TODO section for more notes

## Installing

Add to your `Cargo.toml`

    [dependencies.msgflo]
    git = "https://github.com/msgflo/msgflo-rust"

## API

See [./examples](./examples)

## License

[MIT](./LICENSE.md)

## TODO

0.1

* Allow any number of in/outports
* Allow sending multiple messages out
* Finalize initial API
* Make error handling sane
* Add example of consuming/sending JSON data
* Add test for NACK on error condition
* Publish as Rust crate on http://crates.io

Later

* Support 'hidden' ports (not associated with a queue), used as proxies
* Support other transports, like MQTT

