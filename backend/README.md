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

## License

The backend server of this project is licensed under the AGPLv3 license.

See more in the [License file](../LICENSE-AGPL-3).

The [discord_lib](./discord_lib/README.md) is licensed under the [MIT license](./discord_lib/LICENSE-MIT).