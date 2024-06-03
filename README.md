# membrs

Self-hosted OAuth Discord bot for managing Discord members.

You can find the detailed readme's in the project folders.

![](./preview/msedge_YJFwwVlzxc.gif)

## Quick Installation

For those who prefer a streamlined setup, follow these steps to quickly install the application:

1. Clone the Repository:
    ```shell
    git clone https://github.com/Arteiii/membrs.git
    cd membrs
    ```

2. Set Up Environment Variables:
    ```shell
    cp example.env .env
    vim .env
    ```

3. Build and Start the Application:
    ```shell
    podman compose up
    ```

For more detailed installation instructions, please refer to the [INSTALL.md](INSTALL.md) file.


These files contain more specific details which may be unnecessary for common usage but can be useful for development
and advanced configuration

- [Backend](backend/README.md)
- [Frontend](frontend/README.md)

> [!NOTE]
> This project is not even close to perfect.  
> If you have any tips, improvement ideas, or feature requests, please submit them via [GitHub issues](https://github.com/Arteiii/membrs/issues/new).

## License

Please note that the [discord_lib](/backend/discord_lib/README.md) is licensed under the MIT license.

this project is licensed under the AGPLv3 license

see more in the [License file](LICENSE-AGPL-3).
