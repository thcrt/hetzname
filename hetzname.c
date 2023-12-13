/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#include <getopt.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <cjson/cJSON.h>
#include <curl/curl.h>

CURL *curl;

void error(char message[]) {
    fprintf(stderr, "Hetzname: ERROR: %s\n", message);
    curl_easy_cleanup(curl);
    curl_global_cleanup();
    exit(1);
}

void help() {
    printf(
        "NAME                                                                                   \n"
        "  hetzname - dynamic DNS client for Hetzner                                            \n"
        "                                                                                       \n"
        "SYNOPSIS                                                                               \n"
        "  hetzname {-z ZONE_NAME | -Z ZONE_ID} -r RECORD_NAME [-R RECORD_ID] [-t TTL] [-T TYPE]\n"
        "  hetzname {-z ZONE_NAME | -Z ZONE_ID} [-r RECORD_NAME] -R RECORD_ID [-t TTL] [-T TYPE]\n"
        "  hetzname {-z ZONE_NAME | -Z ZONE_ID} -r RECORD_NAME -r RECORD_ID [-t TTL] [-T TYPE]  \n"
        "                                                                                       \n"
        "DESCRIPTION                                                                            \n"
        "  Idempotently ensure a Hetzner DNS record is set to this computer's public IP address.\n"
        "  A record will be created if it does not exist. A zone for the record must be         \n"
        "  specified either with a ZONE_NAME or ZONE_ID. An individual record must be specified \n"
        "  with a RECORD_NAME, a RECORD_ID or both.                                             \n"
        "                                                                                       \n"
        "  -r RECORD_NAME                                                                       \n"
        "      The name of a record to update. This is usually a subdomain. If a RECORD_ID is   \n"
        "      also passed, the updated record will have its name set to RECORD_NAME. If only a \n"
        "      RECORD_NAME is passed, an existing record with that name will be updated, or     \n"
        "      created if it doesn't exist.                                                     \n"
        "                                                                                       \n"
        "  -R RECORD_ID                                                                         \n"
        "      The ID of a record to update. This can be found through Hetzner's API.           \n"
        "                                                                                       \n"
        "  -t TTL                                                                               \n"
        "      The Time-To-Live value that should be set in the record. If this option is       \n"
        "      omitted then the default value set for the zone will be used.                    \n"
        "                                                                                       \n"
        "   -T TYPE                                                                             \n"
        "      The record type to use, either 'A' for IPv4 or 'AAAA' for IPv6. Defaults to 'A'. \n"
        "                                                                                       \n"
        "  -z ZONE_NAME                                                                         \n"
        "      The name of a zone to operate in. This is usually an apex domain.                \n"
        "                                                                                       \n"
        "  -Z ZONE_ID                                                                           \n"
        "      The ID of a zone to operate in. This can be found through Hetzner's API.         \n"
        "                                                                                       \n"
        "EXAMPLES                                                                               \n"
        "    hetzname -z example.com -r dyn -T AAAA                                             \n"
        "        Updates the 'dyn' record for the zone 'example.com' to an AAAA record with the \n"
        "        value of this computer's current external IPv6 address.                        \n"
        "                                                                                       \n"
        "    hetzname -Z fdnjsks2345 -R dnsklfnsfewihf -r dynamic -t 500                        \n"
        "        Updates a record specified by ID for a zone specified by ID to an A record with\n"
        "        the name 'dynamic', the value of this computer's current external IPv4 address \n"
        "        and time-to-live of 500 seconds.                                               \n"
        "                                                                                       \n"
        "    hetzname -z example.com -R 2ndjsaff3                                               \n"
        "        Updates a record specified by ID for the zone 'example.com' to an A record with\n"
        "        the value of this computer's current external IPv4 address.                    \n"
        "                                                                                       \n"
        "AUTHOR                                                                                 \n"
        "    Written by Theo Court and other contributors.                                      \n"
        "    Inspired by work from FarrowStrange. Built using the Hetzner DNS API.              \n"
        "                                                                                       \n"
        "CONTRIBUTING                                                                           \n"
        "    Report issues and suggest features on GitHub:                                      \n"
        "    <https://github.com/thcrt/hetzname>                                                \n"
        "                                                                                       \n"
        "COPYRIGHT                                                                              \n"
        "    Copyright (c) 2023 Theo Court and other contributors. Licensed under the Mozilla   \n"
        "    License 2.0: <https://www.mozilla.org/en-US/MPL/2.0/>. There is NO WARRANTY, to the\n"
        "    extent permitted by law.                                                           \n"
        "                                                                                       "
        "\n");
}

struct APIResponse {
    char *data;
    size_t size;
};

static size_t APICallback(void *chunk, size_t size, size_t chunk_size,
                          struct APIResponse *response) {
    size_t real_size = size * chunk_size;

    response->data = realloc(response->data, response->size + real_size + 1);
    if (!response->data)
        error("Out of memory while fetching API response!");

    memcpy(&(response->data[response->size]), chunk, real_size);
    response->size += real_size;
    response->data[response->size] = 0;

    return real_size;
}

const char *get_zone_id(char target_name[], char api_token[]) {
    curl_easy_setopt(curl, CURLOPT_URL, "https://dns.hetzner.com/api/v1/zones");

    // Construct and add API token header
    struct curl_slist *headers = NULL;
    char *auth_header;
    asprintf(&auth_header, "Auth-API-Token: %s", api_token);
    headers = curl_slist_append(headers, auth_header);
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
    
    // Prepare a response object and add the callback function
    struct APIResponse response;
    response.data = malloc(1);
    response.size = 0;
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, APICallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, (void *)&response);
    
    // Perform the request
    CURLcode res;
    res = curl_easy_perform(curl);
    if (res != CURLE_OK) {
        char *message;
        asprintf(&message, "API request failed: %s", curl_easy_strerror(res));
        free(response.data);
        curl_slist_free_all(headers);
        error(message);
    }

    // parse JSON and find relevant zone by name
    cJSON *data = cJSON_Parse(response.data);
    cJSON *zones = cJSON_GetObjectItemCaseSensitive(data, "zones");
    cJSON *zone;
    static char zone_id[22];
    cJSON_ArrayForEach(zone, zones) {
        char* name = cJSON_GetObjectItemCaseSensitive(zone, "name")->valuestring;
        if (strcmp(name, target_name) == 0) {
            strcpy(zone_id, cJSON_GetObjectItemCaseSensitive(zone, "id")->valuestring);
        }
    }

    free(response.data);
    curl_slist_free_all(headers);

    if (!zone_id[0]) {
        char *message;
        asprintf(&message, "Can't find zone with name: %s", target_name);
        error(message);
    }

    return zone_id;
}

const char *get_record_id(char zone_name[]) { return "boing"; }

const char *get_zone_name(char zone_name[]) { return "click"; }

const char *get_record_name(char zone_name[]) { return "p'tang"; }

int main(int argc, char *argv[]) {
    char zone_name[256] = "", record_name[256] = "", zone_id[256] = "", record_id[256] = "";
    char record_type[5] = "A";
    int ttl = -1;

    // show help if no arguments are given
    if (argc == 1) {
        help();
        exit(1);
    }

    // Put all passed arguments into the correct variables
    int c;
    while ((c = getopt(argc, argv, "Z:R:z:r:t:T:h")) != -1) {
        switch (c) {
            case 'Z':
                strcpy(zone_id, optarg);
                break;
            case 'R':
                strcpy(record_id, optarg);
                break;
            case 'z':
                if (zone_id[0]) { // zone id was also passed
                    error("Must specify zone ID or zone name, not both!");
                }
                strcpy(zone_name, optarg);
                break;
            case 'r':
                strcpy(record_name, optarg);
                break;
            case 't':
                ttl = atoi(optarg);
                break;
            case 'T':
                if (strcmp(optarg, "A") == 0 || strcmp(optarg, "AAAA") == 0) {
                    strcpy(record_type, optarg);
                } else {
                    char *message;
                    asprintf(&message, "Record type must be 'A' or 'AAAA', not '%s'!", record_type);
                    error(message);
                }
                break;
            case 'h':
                help();
                exit(0);
            default:
                exit(1);
        }
    }

    char *api_token = getenv("HETZNAME_API_TOKEN");
    if (!api_token)
        error("No API token provided! Set the environment variable "
              "HETZNAME_API_TOKEN and try again.");

    curl_global_init(CURL_GLOBAL_ALL);
    curl = curl_easy_init();
    if (!curl)
        error("Error initialising cURL!");

    // fetch missing IDs/names from the API if needed
    if (!zone_id[0])
        strcpy(zone_id, get_zone_id(zone_name, api_token));
    else if (!zone_name[0])
        strcpy(zone_name, get_zone_name(zone_id));
    if (record_name[0] && !record_id[0])
        strcpy(record_id, get_record_id(record_name));
    else if (record_id[0] && !record_name[0])
        strcpy(record_name, get_record_name(record_id));

    printf("Zone name:      '%s'\n", zone_name);
    printf("Zone ID:        '%s'\n", zone_id);
    printf("Record name:    '%s'\n", record_name);
    printf("Record ID:      '%s'\n", record_id);
    printf("Record type:    '%s'\n", record_type);
    printf("Record TTL:     '%d'\n", ttl);
    printf("API token:      '%s'\n", api_token);

    curl_easy_cleanup(curl);
    curl_global_cleanup();
    return 0;
}