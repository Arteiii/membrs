#!/bin/bash

# Download the application
curl -O https://github.com/arteiii/membrs/releases/latest/download/membrs

# Make the downloaded file executable
chmod +x membrs

# Execute the application with sudo
sudo ./membrs
