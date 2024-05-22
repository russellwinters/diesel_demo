# DIESEL DEMO

A demo application showcasing the usage of the Rust library Diesel. Diesel is a powerful and flexible ORM (Object-Relational Mapping) and query builder for Rust, designed to make it easy to interact with databases.

This project is based on a tutorial and may eventually expand on the basic tutorial to cover more advanced topics.

## Installation

Instructions on how to install and set up your project:

1. Make sure you have Rust installed on your system. You can download and install Rust from the official website: https://www.rust-lang.org/tools/install

2. Create an `.env` file in the root directory of your project. This file should contain a `DATABASE_URL` variable that matches the connection URL of your running database. For example:

   ```
   DATABASE_URL=postgres://username:password@localhost/mydatabase
   ```

   Replace `username`, `password`, `localhost`, and `mydatabase` with the appropriate values for your database.

3. Run the following command to build and run the project:

   ```
   cargo run
   ```

   This will compile and execute your project, connecting to the database specified in the `DATABASE_URL` variable.

   Note: Make sure your database server is running before executing the project.
