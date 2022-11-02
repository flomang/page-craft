### actix-web-realworld-example-backend 

There and Backend again! 

A basic api backend tale. 

Featuring:
* login
* logout
* register
* invite

### stack
* actix-web
* actix-identity
* diesel

### diesel migrations

Set the env var database url for diesel cli:
```
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
```

To create migrations:
```
diesel migration generate create_tables
```

To run migrations:
```
diesel setup
```
or
```
diesel migration run 
```
or
```
diesel migration revert
```