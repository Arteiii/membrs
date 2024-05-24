# membrs

selfhosted oauth discord bot for managing discord members

you can find the detailed readmes in the project folders

![](./preview/msedge_YJFwwVlzxc.gif)

## Install

to set up the latest release, you can use the executable from the GitHub Release page


> [!TIP]
> the installer currently only supports linux and also there not every distro
>
> pull request or ideas how to automate the installation on widows are welcome as well as support for more distros so
> feel free to create a issue to check that


to download and directly run it, you can use:

```shell
curl -O -L https://github.com/arteiii/membrs/releases/latest/download/membrs && chmod +x membrs && sudo ./membrs
```

## Manual Install

the app is tested and developed using podman
it should work seamlessly with docker too, tho I prefer podman but that's up to you

before you start the application make sure to set the .env variables from the [`example.env`](./example.env) file and
rename it to the .env instead of example.env

after that you can build and start the containers
this will create a new volume for the database

```shell
docker compose up -d # -d for detached
```

you can visit the admin dashboard at /admin

> [!IMPORTANT]  
> the default username/password is: admin:admin
> I recommend changing asap

To complete the configuration, you need to insert additional data. 
For detailed instructions on setting up Discord applications, refer to the [GitHub Wiki](https://github.com/Arteiii/membrs/wiki/Discord-Application).

For more detailed information about the backend, frontend, and installer, refer to the README files in their respective
directories.
These files contain more specific details which may be unnecessary for common usage but can be useful for development
and advanced configuration.

- [Backend](backend/README.md)
- [Frontend](frontend/README.md)
- [Installer](installer/README.md)

## Description

As I am incredibly annoyed by projects that are hard to set up,
I will do my best to make it as easy as possible.
Currently, planned is an installer (membrs bin) that installs all dependencies and prerequisites for you.


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
