## MVC Diesel Example

This example is a [sapper](https://github.com/sappworks/sapper) app, with diesel as its orm.

This example contains:

1. A model Blog;
2. CRUD on Blog;
3. simple front web pages.


This example is designed to demostrate some sapper's abilities:

1. global wrapper (app middleware);
2. module;
3. module middleware;
4. router;
5. global (cross requests) shared object without unsafe;
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
DATABASE_URL=postgres://postgres:123456@localhost/sapper_diesel
```

You should modify this line to your own config.

### 3. Install Diesel Tool

After setting db config, run

```
cargo install diesel_cli
```

### 4. Do DB Setup

In root of this example, run 

```
diesel setup
```

This will create our database (if it didn't already exist), and create an empty migrations directory that we can use to manage our schema.


### 5. Migration

In root of this example, run 

```
diesel migration run
```

This step will create **blogs** table in your db.


### 6. Run App

run 

```
cargo run
```

to run this example. It will listen on `http://127.0.0.1:1337`.

### 7. Visit Web Page

Open `http://127.0.0.1:1337` to play. 

Good luck! :)

