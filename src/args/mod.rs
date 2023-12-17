// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use argh::FromArgs;

mod record;
mod zone;

/// a command-line client for the Hetzner DNS API
#[derive(FromArgs)]
#[argh(help_triggers("-h", "--help"))]
pub struct Args {
    /// show the current version
    #[argh(switch, short = 'v')]
    pub version: bool,

    #[argh(subcommand)]
    action: Option<Actions>,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Actions {
    Zone(zone::Args),
    Record(record::Args),
}
