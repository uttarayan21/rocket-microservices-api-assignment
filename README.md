# Microservices api assignment

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
Including the databases ( postgresql )
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

