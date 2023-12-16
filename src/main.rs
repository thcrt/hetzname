/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/
use argh::FromArgs;
use std::str::FromStr;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");


enum RecordType { A, AAAA, CAA, CNAME, MX, NS, SRV, TXT }
impl FromStr for RecordType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A"|"a" => Ok(RecordType::A),
            "AAAA"|"aaaa" => Ok(RecordType::AAAA),
            "CAA"|"caa" => Ok(RecordType::CAA),
            "CNAME"|"cname" => Ok(RecordType::CNAME),
            "MX"|"mx" => Ok(RecordType::MX),
            "NS"|"ns" => Ok(RecordType::NS),
            "SRV"|"srv" => Ok(RecordType::SRV),
            "TXT"|"txt" => Ok(RecordType::TXT),
            _ => Err("invalid record type".to_string()),
        }
    }
}



/// a command-line client for the Hetzner DNS API
#[derive(FromArgs)]
#[argh(help_triggers("-h", "--help"))]
struct Args {
    /// show the current version
    #[argh(switch, short = 'v')]
    version: bool,

    #[argh(subcommand)]
    action: Option<Args_Actions>,
}
#[derive(FromArgs)]
#[argh(subcommand)]
enum Args_Actions {
    Zone(ArgsZone),
    Record(ArgsRecord),
}

/// interact with zones
#[derive(FromArgs)]
#[argh(subcommand, name = "zone")]
struct ArgsZone {
    #[argh(subcommand)]
    action: ArgsZone_Actions,
}
#[derive(FromArgs)]
#[argh(subcommand)]
enum ArgsZone_Actions {
    List(ArgsZoneList),
    Get(ArgsZoneGet),
    Create(ArgsZoneCreate),
    Update(ArgsZoneUpdate),
    Delete(ArgsZoneDelete),
    Import(ArgsZoneImport),
    Export(ArgsZoneExport),
    Validate(ArgsZoneValidate)
}

/// list all zones in the account
#[derive(FromArgs)]
#[argh(subcommand, name = "list")]
struct ArgsZoneList {}

/// get information about a zone
#[derive(FromArgs)]
#[argh(subcommand, name = "get")]
struct ArgsZoneGet {
    /// the ID of the zone to get
    #[argh(positional)]
    id: String,
}

/// create a new zone
#[derive(FromArgs)]
#[argh(subcommand, name = "create")]
struct ArgsZoneCreate {
    /// set the name
    #[argh(positional)]
    name: String,

    /// set the Time-To-Live
    #[argh(option, short = 'T')]
    ttl: Option<usize>,
}

/// update a zone
#[derive(FromArgs)]
#[argh(subcommand, name = "update")]
struct ArgsZoneUpdate {
    /// the ID of the zone to update
    #[argh(positional)]
    id: String,

    /// set the name
    #[argh(option, short = 'n')]
    name: Option<String>,

    /// set the Time-To-Live
    #[argh(option, short = 'T')]
    ttl: Option<usize>,
}

/// delete a zone
#[derive(FromArgs)]
#[argh(subcommand, name = "delete")]
struct ArgsZoneDelete {
    /// the ID of the zone to delete
    #[argh(positional)]
    id: String,

    /// don't prompt for confirmation
    #[argh(option)]
    yes_really_delete: bool,
}

/// import a zone file into a zone
#[derive(FromArgs)]
#[argh(subcommand, name = "import")]
struct ArgsZoneImport {
    /// the ID of the zone to import into
    #[argh(positional)]
    id: String,

    /// the zone file to import
    #[argh(positional)]
    file: String,
}

/// export a zone's configuration as a zone file
#[derive(FromArgs)]
#[argh(subcommand, name = "export")]
struct ArgsZoneExport {
    /// the ID of the zone to export
    #[argh(positional)]
    id: String,

    /// the path at which to create the exported zone file
    #[argh(positional)]
    file: String,
}

/// validate a zone file for a zone without importing it
#[derive(FromArgs)]
#[argh(subcommand, name = "validate")]
struct ArgsZoneValidate {
    /// the ID of the zone to validate for
    #[argh(positional)]
    id: String,

    /// the zone file to validate
    #[argh(positional)]
    file: String,
}

/// interact with records within a zone
#[derive(FromArgs)]
#[argh(subcommand, name = "record")]
struct ArgsRecord {
    /// the ID of the zone to operate in
    #[argh(positional)]
    zone: String,

    /// the action to take
    #[argh(subcommand)]
    action: ArgsRecord_Actions,
}
#[derive(FromArgs)]
#[argh(subcommand)]
enum ArgsRecord_Actions {
    List(ArgsRecordList),
    Get(ArgsRecordGet),
    Create(ArgsRecordCreate),
    Update(ArgsRecordUpdate),
    Delete(ArgsRecordDelete),
}

/// list all records
#[derive(FromArgs)]
#[argh(subcommand, name = "list")]
struct ArgsRecordList {}

/// get information about a record
#[derive(FromArgs)]
#[argh(subcommand, name = "get")]
struct ArgsRecordGet {
    /// the ID of the record to get
    #[argh(positional)]
    id: String,
}

/// create a new record
#[derive(FromArgs)]
#[argh(subcommand, name = "create")]
struct ArgsRecordCreate {
    /// set the name
    #[argh(positional)]
    name: String,

    /// set the type (A, AAAA, CAA, CNAME, MX, NS, SRV, TXT)
    #[argh(option, short = 't', default = "RecordType::A")]
    type_: RecordType,

    /// set the Time-To-Live
    #[argh(option, short = 'T')]
    ttl: Option<usize>,

    /// set the value to the host's current public IP address (needs a record type of A or AAAA)
    #[argh(option, short = 'd')]
    ddns: bool,

    /// set the value to the given string
    #[argh(option, short = 'v')]
    value: String,
}

/// update a record
#[derive(FromArgs)]
#[argh(subcommand, name = "update")]
struct ArgsRecordUpdate {
    /// the ID of the record to update
    #[argh(positional)]
    id: String,

    /// set the name
    #[argh(option, short = 'n')]
    name: Option<String>,

    /// set the type (A, AAAA, CAA, CNAME, MX, NS, SRV, TXT)
    #[argh(option, short = 't')]
    type_: Option<RecordType>,

    /// set the Time-To-Live
    #[argh(option, short = 'T')]
    ttl: Option<usize>,

    /// set the value to the host's current public IP address (needs a record type of A or AAAA)
    #[argh(option, short = 'd')]
    ddns: bool,

    /// set the value to the given string
    #[argh(option, short = 'v')]
    value: String,
}

/// delete a record
#[derive(FromArgs)]
#[argh(subcommand, name = "delete")]
struct ArgsRecordDelete {
    /// the ID of the record to delete
    #[argh(positional)]
    id: String,

    /// don't prompt for confirmation
    #[argh(option)]
    yes_really_delete: bool,
}


fn main() {
    let args: Args = argh::from_env();

    if args.version {
        println!("Hetzname version {}", VERSION.unwrap_or("unknown"));
    }
}