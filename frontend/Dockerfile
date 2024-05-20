# Use the official Node.js image
FROM node:18-alpine AS builder

# Create and set the working directory
WORKDIR /app

# Copy the package.json and package-lock.json files
COPY package*.json ./

# Install the dependencies
RUN npm install

# Copy the rest of the application code
COPY . .

# Build the application
RUN npm run build

# Use a minimal base image for the final stage
FROM node:16-alpine

# Set the working directory
WORKDIR /app

# Copy the built application from the builder stage
COPY --from=builder /app ./

# Install serve to serve the built application
RUN npm run build

# Expose the port the app runs on
EXPOSE 3000

# Specify the command to run the application
CMD ["npm", "run", "start"]
