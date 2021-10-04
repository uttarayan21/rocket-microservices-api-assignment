# Microservices api assignment

## Running the code
docker and docker-compose needs to be setup first
```bash
cd user_service && docker-compose up -d && cd ..
cd user_interaction_service && docker-compose up -d && cd ..
cd content_service && docker-compose up -d && cd ..
```
_NOTE: This will take a fair bit of time since rust is a compiled language it needs a few minutes to compile_

## Ingesting data
The scripts folder has a populate.sh script that can populate the databases with dummy data as long as nothing is modified beforehand
```bash
cd scripts
./populate.sh
```

_Note: The content service can ingest csv file with 'Content-Type: text/csv but the dates **need** to be a unix timestamp_ 


## API endpoints
Since there is no service discovery I have hardcoded the values into each microservices .env file

- user_service -> `localhost:8001`
- content_service -> `localhost:8002`
- user_interaction_service -> `localhost:8003`

## Documented with openapi/swagger
Every route is documented with openapi/swagger specification and can be found after running the api and visiting 'address/swagger'
The documentation is automatically generated during compilation and doesn't need to be manually generated.

- user_service -> `localhost:8001/swagger`
- content_service -> `localhost:8002/swagger`
- user_interaction_service -> `localhost:8003/swagger`


## Dockerized everyting
Including the databases (postgresql)

Each service has a builder (rust:latest) which compiles the code and then runs it via a runner (debian:stable-slim) and a postgresql (postgresql:latest) container for the database.
So thats 2 containers for each service
    - user-service-api
        - [x] user_service
        - [x] user_postgres_service  
    - content-service-api
        - [x] content_service
        - [x] content_postgres_service
    - user-interaction-service-api
        - [x] user_interaction_service
        - [x] user_interaction_postgres_service

## Microservices
Everything is a microservice with 3 separate instances of postgresql running and obviously 3 separate databases.


## About the codebase
This is build using [rust](https://www.rust-lang.org/) language and [rocket](https://rocket.rs) framework, using [diesel](https://diesel.rs) ORM and [postgresql](https://www.postgresql.org/) database.

