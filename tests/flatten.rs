// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Andrew Hobden (@hoverbear) <andrew@hoverbear.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// This work was derived from Structopt (https://github.com/TeXitoi/structopt)
// commit#ea76fa1b1b273e65e3b0b1046643715b49bec51f which is licensed under the
// MIT/Apache 2.0 license.

#[macro_use]
extern crate clap;

use clap::Clap;

#[test]
fn flatten() {
    #[derive(Clap, PartialEq, Debug)]
    struct Common {
        arg: i32,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(flatten)]
        common: Common,
    }
    assert_eq!(
        Opt {
            common: Common { arg: 42 }
        },
        Opt::parse_from(&["test", "42"])
    );
    assert!(Opt::try_parse_from(&["test"]).is_err());
    assert!(Opt::try_parse_from(&["test", "42", "24"]).is_err());
}

#[test]
#[should_panic]
fn flatten_twice() {
    #[derive(Clap, PartialEq, Debug)]
    struct Common {
        arg: i32,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Opt {
        #[clap(flatten)]
        c1: Common,
        // Defines "arg" twice, so this should not work.
        #[clap(flatten)]
        c2: Common,
    }
    Opt::parse_from(&["test", "42", "43"]);
}

#[test]
fn flatten_in_subcommand() {
    #[derive(Clap, PartialEq, Debug)]
    struct Common {
        arg: i32,
    }

    #[derive(Clap, PartialEq, Debug)]
    struct Add {
        #[clap(short = "i")]
        interactive: bool,
        #[clap(flatten)]
        common: Common,
    }

    #[derive(Clap, PartialEq, Debug)]
    enum Opt {
        #[clap(name = "fetch")]
        Fetch {
            #[clap(short = "a")]
            all: bool,
            #[clap(flatten)]
            common: Common,
        },

        #[clap(name = "add")]
        Add(Add),
    }

    assert_eq!(
        Opt::Fetch {
            all: false,
            common: Common { arg: 42 }
        },
        Opt::parse_from(&["test", "fetch", "42"])
    );
    assert_eq!(
        Opt::Add(Add {
            interactive: true,
            common: Common { arg: 43 }
        }),
        Opt::parse_from(&["test", "add", "-i", "43"])
    );
}
