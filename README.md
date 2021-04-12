

### Notes
*  `sudo apt install libpq-dev` für  `cargo install diesel_cli`


### Codegen
To generate code by using the openapi 3.0 specification, use the following command:
```
docker run --rm \
      -v ${PWD}:/local openapitools/openapi-generator-cli generate \
      -i /local/src/openapi_server/openapi/openapi.yaml \
      -g rust-server \
      --skip-overwrite \
      -o /local/src
```