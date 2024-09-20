# ZiiZ Backend

A modern, efficient and fast payment gateway backend, with it's main API built in Rust for performance and safety, and a bunch of micro-services.

## Project folder structure

```
ziiz-backend/
│
├── api/                            # Rust API microservice
│   ├── src/
│   ├── Cargo.toml
│   ├── Dockerfile
│   ├── .env
│   └── README.md
│
├── exchange-rates/                 # Exchange rate service (either microservice or worker)
│   ├── src/
│   ├── Cargo.toml
│   ├── Dockerfile
│   ├── .env
│   └── README.md
│
├── redis/                          # Redis config
│   ├── redis.conf
│   └── README.md
│
├── monitoring/                     # Monitoring tools (Prometheus + Grafana)
│   ├── prometheus/
│   │   ├── prometheus.yml
│   │   └── Dockerfile
│   │
│   ├── grafana/
│   │   └── Dockerfile
│   └── README.md
│
├── docker-compose.yml              # Docker Compose config for local dev
├── .env                            # Global environment variables for Docker Compose
├── .gitignore
└── README.md
```
