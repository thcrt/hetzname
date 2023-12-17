// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod args;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    let args: args::Args = argh::from_env();

    if args.version {
        println!("Hetzname version {}", VERSION.unwrap_or("unknown"));
    }
}
