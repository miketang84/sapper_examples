## MVC_Example

MVC example is the first full featured [sapper](https://github.com/sappworks/sapper) app.

This example contains:

1. A model Blog;
2. CRUD on Blog;
3. simple front web pages.


This example is designed to demostrate some sapper's abilities:

1. global wrapper (app middleware);
2. module;
3. module middleware;
4. router;
5. global (cross requests) shared object not using unsafe;
6. orm;
7. orm codegen;
8. create/drop db table from command;
9. template rendering;
10. helper macros;
11. redirection;

## How to run it

### 1. Install Postgresql 

First, you need install postgresql (I tested it on postgresql 9.4).

### 2. Modify .env Config File

Modify `.env` file in the root of this example, my setting is like this:

```
DB_URL=postgres://postgres:123456@localhost:5432/postgres
```

You should modify this line to your own config.

### 3. Create DB Table

After setting db config, run

```
cargo run --bin create_table
```

This will create `Blog` table in your pg database;

(you can run `cargo run --bin drop_table` if you want to clean those table data)

### 4. Auto Generate Model Code

run 

```
./generate_model.sh
``` 

to generate model code from postgresql automatically.

### 5. Run App

run 

```
cargo run --bin mvc_example
```

to run this example. It will listen on `http://127.0.0.1:1337`.

### 5. Visit Web Page

Open `http://127.0.0.1:1337` to play. 

Good luck! :)

