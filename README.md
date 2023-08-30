# Hetzname
**_[DDNS](https://en.wikipedia.org/wiki/Dynamic_DNS) for Hetzner's DNS Console, using the [official API](https://dns.hetzner.com/api-docs) and minimal dependencies._**
 
This is a fork of [FarrowStrange](https://github.com/FarrowStrange)'s [`hetzner-api-dyndns`](https://github.com/FarrowStrange/hetzner-api-dyndns). 
Features from Hetzname will be submitted upstream, and any new features upstream will naturally be merged here in due time.
I'm maintaining this fork for my own use and to help anyone else in the same situation as me.

There currently exist a shell script (capable of running in `sh`) and a Python script. Only the shell script is presently maintained/supported, but this should change shortly.
All instructions in this document refer to the shell script unless otherwise stated.

## Preparation

### Dependencies
One goal of Hetzname is to keep the list of dependencies as light as possible. Please feel free to suggest alternate libraries or approaches. 
- [`curl`](https://curl.se/)
- [`jq`](https://stedolan.github.io/jq/)

### Access Token
Use of the API requires an access token, which can be generated [here](https://dns.hetzner.com/settings/api-token). Set a name for the token that will identify it to you, such as `hetzname` or `dyndns-<name of your router>`. Copy it and store it safely, as it will only be displayed once, directly after generation.

Before using the script, set the Access Token as an accessible variable named `HETZNER_AUTH_API_TOKEN`:
```sh
# To have the variable persist throughout the session
export HETZNER_AUTH_API_TOKEN=mysupersecrettoken
hetzname --my-options

# To only make it available to this instance of the program
HETZNER_AUTH_API_TOKEN=mysupersecrettoken hetzname --my-options
```

### Installation
Download the shell script and place it somewhere in your path, like `/usr/local/bin`. Make sure it's executable with `chmod +x /usr/local/bin/hetzname`. 

## Usage
### Arguments
The syntax is described below:
```
hetzname {-z <zone ID> | -Z <zone name> } { -r <record ID> | -n <record name> } [ -t <TTL in seconds> ] [ -T < A | AAAA > ]
```
The zone and record must both be specified. The zone can be identified with a name or an ID (i.e. `example.com` or `98jFjsd8dh1GHasdf7a8hJG7`, respectively). 
The record must be given a name, and may be identified an ID. Should a record exist with the specified ID but a different name, the record's name will be updated to match the given name.

TTL (Time-To-Live) can be specified in seconds with `-t`, and defaults to `60`. Should your external IP address change regularly, it is advisable to set this to a relatively low value.
The record type defaults to `A` (IPv4), but can be set to `AAAA` (IPv6) with `-T AAAA`.

A help menu can be viewed by passing the flag `-h`.

### Environment variables
Instead of command-line arguments, the following environment variables will be used by Hetzname if set:

|NAME                     | Value                              | Description                                                        |
|:------------------------|------------------------------------|:-------------------------------------------------------------------|
|`HETZNER_AUTH_API_TOKEN` | `925bf046408b55c313740eef2bc18b1e` | Your Hetzner API access token.                                     |
|`HETZNER_ZONE_NAME`      | `example.com`                      | The zone name. Mutually exclusive with `HETZNER_ZONE_ID`.          |
|`HETZNER_ZONE_ID`        | `DaGaoE6YzDTQHKxrtzfkTx`           | The zone ID. Mutually exclusive with `HETZNER_ZONE_NAME`.          |
|`HETZNER_RECORD_NAME`    | `dyn`                              | The record name. `'@'` to set an apex record.                      |
|`HETZNER_RECORD_TTL`     | `120`                              | The TTL (Time-To-Live) of the record in seconds. Default `60`.     |
|`HETZNER_RECORD_TYPE`    | `AAAA`                             | The record type, either `A` (IPv4) or `AAAA` (IPv6). Default `A`.  |

### Regular updates
The script is not a daemon, and only runs once when called. It can be called on a regular basis through use of a cron job. 

The following cron entry will run Hetzname every 5 minutes:
```
*/5 * * * * HETZNER_AUTH_API_TOKEN='mysupersecrettoken' hetzname -Z example.com -n dyn
```

Note that the script checks any pre-existing value of the specified record, and only sends an update request if it doesn't match the external IP. 

## Miscellaneous API queries
The following is a collection of potentially useful API queries that can be run with `curl`. 
Where variables such as `${apitoken}` are listed, they may be set beforehand or substituted for the appropriate values.

### List zones
To get all zones in your account, including Zone IDs:
```
curl "https://dns.hetzner.com/api/v1/zones" -H \
'Auth-API-Token: ${apitoken}' | jq
```

### Find record ID
To find the record ID of an already-existing record:
```
curl -s --location \
    --request GET 'https://dns.hetzner.com/api/v1/records?zone_id='${zone_id} \
    --header 'Auth-API-Token: '${apitoken} | \
    jq --raw-output '.records[] | select(.type == "'${record_type}'") | select(.name == "'{record_name}'") | .id'
```
