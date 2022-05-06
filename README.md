# User management and authentication micro-service
This worker provides API to both admin and user as well as GRPC interface for other micro-services. You can find the documentation at [docs folder](https://github.com/fum-cloud-project/auth-api/tree/master/docs) in org and html format or by accessing GET /api/docs after running the project.

## Running in development environment
Prior to building the project you must use the .env.sample file to create a .env file with correct information about the development environment. Here you can see some information about these variables.
1. DATABASE_URL : mongodb uri containing user and password as well as the domain name and port
2. REDIS_URL : redis url
3. DOCS_PATH : path to documentation.html
4. WORKER_THREAD_NUM : API server has 3 actors, cache, db and api_handler; the first two each run on one thread but the api_handler can be configured to use more if available.
5. GRPC_SERVER_URL_AND_PORT : this variable lets the grpc_handler know to listen to what port, at what path.
6. API_SERVER_PORT : api_handler port
7. API_SERVER_ADDRESS : api_handler url
8. API_LOG_FILE : path to a file you wish to have generated logs by the api_handler
9. GRPC_LOG_FILE : path to a file you wish to have generated logs by the grpc_handler
10. SALT_STR : salt string for adding to hashed passwords, have in mind that salt is a 22-character Base64 encoding of the 16 bytes of salt. The salt must be exactly this long
11. SECRET : secret string for generating JWT
12. ADMIN_EMAIL : email for auto created admin user
13. ADMIN_PASSWORD : password for auto created admin user
14. RESOURCE : path to json file containing resource definitions

Have in mind that if any of the above is invalid the server will panic and won't run properly. Make sure a mongodb instance and a redis instance is running before you execute the server.
After creating this file all you need to do is to run:

``` sh
cargo build --bin api-server --release
cargo build --bin grpc-server --release
./target/release/api-server & ./target/release/grpc-server
```

## Running using docker-compose
You need to create a similar .env file with valid path and urls withing the docker environment. After that All you need to do is to run:

``` sh
docker-compose build
docker-compose up
```

