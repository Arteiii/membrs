# membrs

Self-hosted OAuth Discord bot for managing Discord members.

You can find the detailed readme's in the project folders.

![](./preview/msedge_YJFwwVlzxc.gif)

## Install

To set up the latest release, you can use the executable from the GitHub release page.

> [!TIP]
> The installer currently only supports Linux, and not every distro.
> 
> Pull requests or ideas on how to automate the installation on widows are welcome, 
> as is support for more distros, so feel free to create an issue to check.


to download and run it directly:

```shell
curl -O -L https://github.com/arteiii/membrs/releases/latest/download/membrs && chmod +x membrs && sudo ./membrs
```

## Manual Install

The application was tested and developed using Podman.
It should also work seamlessly with Docker, although I prefer Podman, but that's up to you.

before you start the application, make sure you set the .env variables from the file [`example.env`](./example.env) and
rename them to .env instead of example.env

Then you can build and run the containers.
This will create a new volume for the database.


```shell
podman compose build && podman compose up
```

you can visit the admin dashboard at /admin

> [!IMPORTANT]  
> the default username/password is: admin:admin
> I recommend changing asap

To complete the configuration, you need to insert additional data. 
For detailed instructions on setting up Discord applications, refer to the [GitHub Wiki](https://github.com/Arteiii/membrs/wiki/Discord-Application).

For more detailed information about the backend and frontend, refer to the README files in their respective
directories.
These files contain more specific details which may be unnecessary for common usage but can be useful for development
and advanced configuration.

- [Backend](backend/README.md)
- [Frontend](frontend/README.md)


> [!NOTE]
> This project is not even close to perfect.
> It is my first ever fullstack web application, and I am using it as a learning project.
> Prior to this, I had never worked with technologies like PostgreSQL, Axum, Next.js, or OAuth with Discord.
>
> Given that this is a learning project, you might encounter bugs, incomplete features, or suboptimal code.
> Your understanding and patience are appreciated.
> If you have any suggestions or improvements, feel free to contribute!

## License

this project is licensed under the AGPLv3 license

see more in the [License file](LICENSE-AGPL-3).
