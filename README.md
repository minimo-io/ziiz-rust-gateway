# ZiiZ Rust Gateway

Reference: https://github.com/robatipoor/rustfulapi/tree/main

## SeaORM entities

When re-generating entities let's remember to add the needed Serialize derive

`sea-orm-cli generate entity -o src/entity --with-serde both`

## Docker (manual for now)

Build the thing
`docker build -t ziiz .`

Run it
`docker run -p 5000:8080 [image_hash]`
