// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use argh::FromArgs;
use std::str::FromStr;

enum RecordType {
    A,
    AAAA,
    CAA,
    CNAME,
    MX,
    NS,
    SRV,
    TXT,
}
impl FromStr for RecordType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "a" => Ok(RecordType::A),
            "AAAA" | "aaaa" => Ok(RecordType::AAAA),
            "CAA" | "caa" => Ok(RecordType::CAA),
            "CNAME" | "cname" => Ok(RecordType::CNAME),
            "MX" | "mx" => Ok(RecordType::MX),
            "NS" | "ns" => Ok(RecordType::NS),
            "SRV" | "srv" => Ok(RecordType::SRV),
            "TXT" | "txt" => Ok(RecordType::TXT),
            _ => Err("invalid record type".to_string()),
        }
    }
}

/// interact with records within a zone
#[derive(FromArgs)]
#[argh(subcommand, name = "record")]
pub struct Args {
    /// the ID of the zone to operate in
    #[argh(positional)]
    zone: String,

    /// the action to take
    #[argh(subcommand)]
    action: Action,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Action {
    List(List),
    Get(Get),
    Create(Create),
    Update(Update),
    Delete(Delete),
}

/// list all records
#[derive(FromArgs)]
#[argh(subcommand, name = "list")]
struct List {}

/// get information about a record
#[derive(FromArgs)]
#[argh(subcommand, name = "get")]
struct Get {
    /// the ID of the record to get
    #[argh(positional)]
    id: String,
}

/// create a new record
#[derive(FromArgs)]
#[argh(subcommand, name = "create")]
struct Create {
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
struct Update {
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
struct Delete {
    /// the ID of the record to delete
    #[argh(positional)]
    id: String,

    /// don't prompt for confirmation
    #[argh(option)]
    yes_really_delete: bool,
}
