# User-Service

## Setup

To run the server you need to start the database:

``` bash
docker run -d -p 27017:27017 --name user_db mongo
```

## Build the project

``` bash
cargo build
```

## Run the server

``` bash
cargo run
```

## Showcase

``` bash
http localhost:13331/user

http POST localhost:13331/user username="Mario"
http POST localhost:13331/user username="Luigi"
http POST localhost:13331/user username="Princess Peach"
http POST localhost:13331/user username="Yoshi"

http localhost:13331/user 

http PUT localhost:13331/user/:id username="Toad"

http localhost:13331/user 

http DELETE localhost:13331/user/:id

http localhost:13331/user
```