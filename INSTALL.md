# Installation

## Manual

### Prerequisites

Before starting, ensure you have the following software installed on your system:

- [Git](https://git-scm.com/) - Version control system
- [Podman](https://podman.io/) - Tool for managing OCI containers (or
  use [Docker](https://docs.docker.com/engine/install/))

#### Installing Prerequisites

- On Debian/Ubuntu:
    ```shell
    sudo apt update
    sudo apt install git podman
    ```

- On Fedora:
    ```shell
    sudo dnf install git podman
    ```

> [!TIP]
> For other systems, refer to the official [Podman installation guide](https://podman.io/docs/installation).

### Cloning the Repository and Setting Up Environment Variables

1. Open the terminal:  
   Open your terminal or command prompt.

2. Clone the repository:  
   Use the git clone command to clone the repository from GitHub.  
    ```shell
    git clone https://github.com/Arteiii/membrs.git
    ```

3. Navigate to the project directory:  
   After cloning the repository, navigate into the newly created project directory.  
    ```shell
    cd membrs
    ```

4. Copy example.env to .env:
   Use the cp command to copy example.env to .env:
    ```shell
    cp example.env .env
    ```

5. Edit the .env file:
    ```shell
    vim .env
    ```

Example .env file:

```makefile
POSTGRES_USER=postgres
POSTGRES_PASSWORD=verys3curep4ss

URL=http://example.com # the domain ur website and backend will be available at
PORT=80 # the port u want to use (>=1024 by default)
```

### Using Podman

#### Building and Starting the Application

1. Build the Docker images:  
   Use the podman compose build command to build the Docker images defined in your docker-compose.yml file:
    ```shell
    podman compose build
    ```

2. Start the services:
   Use the podman compose up command to start the services:
    ```shell
    podman compose up
    ```

#### Important Note on Ports Below 1024

> [!WARNING]  
> Podman, by default, does not run as the root user.
> Therefore, it does not allow non-root processes to bind to ports below 1024,
> which are considered privileged system ports.

Here are a few ways to handle this:

1. Use a Reverse Proxy (e.g., Cloudflare):  
   You can use a reverse proxy to forward traffic from a higher port to port 80.
   This way, Podman can listen on a higher port, and the reverse proxy will handle the requests on port 80.

2. Change Server Configuration to Allow Non-root Processes to Listen on Ports Below 1024:  
   You can configure your system to allow non-root processes to bind to privileged ports. This can be done by setting
   the net.ipv4.ip_unprivileged_port_start parameter.  
   For example, to allow binding to all ports, you can set it to 0:
    ```shell
    sudo sysctl -w net.ipv4.ip_unprivileged_port_start=0
    ```

3. Configure Podman to Run as Root:  
   Running Podman as root will allow it to bind to privileged ports. Use the sudo command to run Podman commands as
   root.

> [!TIP]
> For a comprehensive guide on setting up and using Podman in rootless mode, visit
> the [Rootless Podman Tutorial](https://github.com/containers/podman/blob/main/docs/tutorials/rootless_tutorial.md).

### Summary of Commands

```shell
# Install prerequisites
sudo apt update
sudo apt install git podman  # For Debian/Ubuntu

# Or for Fedora
sudo dnf install git podman

# Or for macOS using Homebrew
brew install git podman

# Clone the repository and set up environment variables
cd path/to/parent/directory
git clone YOUR_GITHUB_REPO_URL
cd membrs
cp example.env .env
vim .env

# In Vim, make your changes, then save and exit
# Build and start the application using Podman
podman compose build
podman compose up
```

## Admin

the Admin dashboard is by default available at `/admin`

> [!WARNING]  
> the default username/password is: admin:admin
> I recommend changing it asap

> [!IMPORTANT]
> To complete the configuration, you need to insert additional data.
> For detailed instructions on setting up Discord applications, refer to
> the [GitHub Wiki](https://github.com/Arteiii/membrs/wiki/Discord-Application).


## Common Issues

### 



