## [Lesson](https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/)


sudo apt install libpq-dev libmariadbclient postgresql-libs sqlite

cargo install diesel_cli --no-default-features --features postgres  --verbose

diesel setup


1. The next step now is to add our migrations using the CLI:

    diesel migration generate add_users


2. We will first edit up.sql to add SQL to create our table. 

3. Create the model 

4. use diesel to automatically generate the schema it needs in a file called schema.rs. 

    diesel print-schema > src/schema.rs

5. Apply our database migration

    diesel migration run