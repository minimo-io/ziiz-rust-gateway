# ZiiZ Rust Gateway

A modern, efficient and fast payment gateway built in Rust.

## SeaORM entities

When re-generating entities let's remember to add the needed Serialize derive

`sea-orm-cli generate entity -o src/entity --with-serde both`

## Docker (manual for now)

Build the thing
`docker build -t ziiz .`

Run it with your local .env file
`docker run -p 5000:8080 --env-file .env [image_hash]`

## To Do

Reference: https://github.com/robatipoor/rustfulapi/tree/main

-   Close database connection on exit
-   Create the final structure for the project (or so)
-   Create a docker compose pipeline
-   Lean how to stress test
    -   Confirm that the database is handling multi-thread Arc<> stuff
-   Better share env variables
-   Documentation
-   Create a config for the server: https://youtu.be/aZmrfizffL0?si=ysS0H7qZIa57hF2Y&t=520
-   Monitoring
