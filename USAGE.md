# Usage Examples

This document provides practical examples for using `idgen` to generate various types of identifiers.

## UUID

### Basic Generation

Generate a random UUID v4 (default):

```sh
$ idgen uuid
9e4a5f33-f01c-47ee-8527-18ec1f0151d5
```

### UUID Versions

Generate a UUID v7 (timestamp-based, sortable):

```sh
$ idgen uuid -v 7
019c5e56-c3ea-7dc3-ba6d-00a7256fdb53
```

Generate a UUID v1 with a custom timestamp:

```sh
$ idgen uuid -v 1 --timestamp 1234567890000000000
70d9b500-fa26-11dd-8000-da81dd7abf20
```

Generate a UUID v5 (name-based with SHA-1):

```sh
$ idgen uuid -v 5 --namespace dns --name example.com
cfbff0d1-9375-5685-968c-48ce8b15ae17
```

Generate a UUID v8 with custom data:

```sh
$ idgen uuid -v 8 --data 0123456789abcdef
01234567-89ab-8def-8000-000000000000
```

### Multiple UUIDs

Generate multiple UUIDs:

```sh
$ idgen -n 5 uuid
355dd7c8-1742-446b-aa2f-2a1f0ebd08bb
acff238d-b8a2-4171-8058-e1c3b0c7d053
c264877e-4966-4233-b36d-370801fa225a
b1ddba48-1ef4-417c-88f0-16dcf50c11e6
6f0bdf32-785e-477f-8545-4c3afdca4b2f
```

## ULID

### Basic Generation

Generate a ULID (timestamp + randomness, sortable, 26 characters):

```sh
$ idgen ulid
01KHF5DXJFPRAM4CX0WPMV21Z9
```

### With Timestamp

Generate a ULID with a specific timestamp (milliseconds):

```sh
$ idgen ulid --timestamp 1609459200000
01ETXKWW00DDW621CQ6QZJF3GV
```

### Multiple ULIDs

Generate multiple ULIDs:

```sh
$ idgen -n 3 ulid
01KHF5DZCBF8XVBHYAV1Z3WSBS
01KHF5DZCC7PPRRKT95YSWJSY7
01KHF5DZCCBFRTTHDHE8H38AXK
```

## ObjectId

### Basic Generation

Generate a MongoDB ObjectId (12-byte identifier, 24 hex characters):

```sh
$ idgen oid
6990fba67a68e4c0fd192bdb
```

### With Timestamp

Generate an ObjectId with a specific timestamp (seconds):

```sh
$ idgen oid --timestamp 1609459200
5fee660060e46b1212c9796e
```

### Using Alias

Use the `objectid` alias:

```sh
$ idgen objectid
6990fba81631f19014909b05
```

## Practical Use Cases

### Shell Scripts

Use in shell scripts:

```sh
#!/bin/bash
USER_ID=$(idgen uuid)
echo "Creating user with ID: $USER_ID"
```

### Test Data Generation

Generate test data:

```sh
for i in {1..100}; do
  echo "INSERT INTO users (id) VALUES ('$(idgen uuid)');"
done > insert_users.sql
```

### File Naming

Generate identifiers for file names:

```sh
BACKUP_FILE="backup-$(idgen ulid).tar.gz"
tar -czf "$BACKUP_FILE" /path/to/data
```

### Bulk Operations

Generate multiple identifiers for different purposes:

```sh
# Generate 10 user IDs
idgen -n 10 uuid > user_ids.txt

# Generate 5 sortable event IDs
idgen -n 5 uuid -v 7 > event_ids.txt

# Generate 20 document IDs for MongoDB
idgen -n 20 oid > document_ids.txt
```
