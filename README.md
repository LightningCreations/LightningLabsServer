# Lightning Labs Server

## Local deployment instructions

Create a PostgreSQL database named "ll-local". Leave it empty, but make sure you have a user that can access it.

Create a `.env` file with the following contents, filling in the brackets with information from earlier:

```
DATABASE_URL=postgres://<username>:<password>@localhost/ll-local
```

Once the environment file is ready, make sure the diesel CLI is installed (if not, run `cargo install diesel_cli`). Then, run:

```
diesel migration run
```

This will set up the database based on the embedded migration files.

Lastly, `cargo run`. You're all set!
