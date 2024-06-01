# membrs (Backend)

requires postgresql

## ENV vars

Make sure to set the following environment variables:

To get log output while the container is running, use the following commands:

```shell
docker logs membrs-backend  # Prints the log history
# or
docker logs -f membrs-backend  # Follow mode keeps streaming the output
```

## Lib

This project includes a basic helper library created to facilitate working with the Discord API.  
Note that there are already more advanced libraries available for working with Discord in Rust. Instead, consider using:
- [serenity](https://github.com/serenity-rs/serenity)  
- [poise](https://github.com/serenity-rs/poise)

## License

this project is licensed under the AGPLv3 license

see more in the [License file](LICENSE-AGPL-3).
