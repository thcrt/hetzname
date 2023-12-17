// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use argh::FromArgs;

/// interact with zones
#[derive(FromArgs)]
#[argh(subcommand, name = "zone")]
pub struct Args {
    #[argh(subcommand)]
    pub action: Action,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Action {
    List(List),
    Get(Get),
    Create(Create),
    Update(Update),
    Delete(Delete),
    Import(Import),
    Export(Export),
    Validate(Validate),
}

/// list all zones in the account
#[derive(FromArgs)]
#[argh(subcommand, name = "list")]
struct List {}

/// get information about a zone
#[derive(FromArgs)]
#[argh(subcommand, name = "get")]
struct Get {
    /// the ID of the zone to get
    #[argh(positional)]
    pub id: String,
}

/// create a new zone
#[derive(FromArgs)]
#[argh(subcommand, name = "create")]
struct Create {
    /// set the name
    #[argh(positional)]
    pub name: String,

    /// set the Time-To-Live
    #[argh(option, short = 'T')]
    pub ttl: Option<usize>,
}

/// update a zone
#[derive(FromArgs)]
#[argh(subcommand, name = "update")]
struct Update {
    /// the ID of the zone to update
    #[argh(positional)]
    pub id: String,

    /// set the name
    #[argh(option, short = 'n')]
    pub name: Option<String>,

    /// set the Time-To-Live
    #[argh(option, short = 'T')]
    pub ttl: Option<usize>,
}

/// delete a zone
#[derive(FromArgs)]
#[argh(subcommand, name = "delete")]
struct Delete {
    /// the ID of the zone to delete
    #[argh(positional)]
    pub id: String,

    /// don't prompt for confirmation
    #[argh(option)]
    pub yes_really_delete: bool,
}

/// import a zone file into a zone
#[derive(FromArgs)]
#[argh(subcommand, name = "import")]
struct Import {
    /// the ID of the zone to import into
    #[argh(positional)]
    pub id: String,

    /// the zone file to import
    #[argh(positional)]
    pub file: String,
}

/// export a zone's configuration as a zone file
#[derive(FromArgs)]
#[argh(subcommand, name = "export")]
struct Export {
    /// the ID of the zone to export
    #[argh(positional)]
    pub id: String,

    /// the path at which to create the exported zone file
    #[argh(positional)]
    pub file: String,
}

/// validate a zone file for a zone without importing it
#[derive(FromArgs)]
#[argh(subcommand, name = "validate")]
struct Validate {
    /// the ID of the zone to validate for
    #[argh(positional)]
    pub id: String,

    /// the zone file to validate
    #[argh(positional)]
    pub file: String,
}
