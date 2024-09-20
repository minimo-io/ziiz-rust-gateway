# ZiiZ Rust Gateway

A modern, efficient and fast payment gateway built in Rust.

## SeaORM entities

When re-generating entities let's remember to add the needed Serialize derive

`sea-orm-cli generate entity -o src/entities --with-serde both`

## Docker (manual for now)

Build the thing
`docker build -t ziiz .`

Run it with your local .env file
`docker run -p 5000:8080 --env-file .env [image_hash]`
`docker run -p 5000:8080 --env-file .env e275e7a2e1200b04f69f2eb9d1d4d04a2a053b628251db8b6d2cf5f458b374c9`

## To Do

Reference: https://github.com/robatipoor/rustfulapi/tree/main

-   Continue creating the final structure for the project.
-   Create a docker compose pipeline:
    -   Use .env files or Docker secrets for managing environment-specific configurations (API keys, DB credentials, etc.).
    -   Ensure that secrets for both the local and live test environment are securely stored, possibly using GitHub Actions Secrets for CI/CD pipelines.
-   Create a config mod for the server: https://youtu.be/aZmrfizffL0?si=ysS0H7qZIa57hF2Y&t=520
-   Better app_state to share among the whole app: https://www.youtube.com/watch?v=ENgFBKTBqDU
-   Close database connection on exit
-   Lean how to stress test
    -   Confirm that the database is handling multi-thread Arc<> stuff
-   Better share env variables
-   Monitoring
-   Documentation: OpenAPI/Swagger (for clients) and Rustdocs (internal).

```
[dependencies]
utoipa = "2.0"
utoipa-swagger-ui = "2.0"  # Optional for self-hosting Swagger UI
```

```
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(api::your_api_function))]
struct ApiDoc;

fn main() {
    let openapi = ApiDoc::openapi();
    println!("{}", openapi.to_pretty_json().unwrap());
}

```

Automate the pipeline via Github Actions to re-generate the docs.

-   (v2) Caching: As the project scales, introduce caching with Redis to reduce the load on your PostgreSQL database.
