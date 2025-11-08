# rust_job_search
Rust Port of an old project of mine

## Migrations
[refinery-cli](https://github.com/rust-db/refinery/tree/main/refinery_cli) is the migrations runner.

### Install refinery-cli
Install with `cargo install refinery_cli`

### Create empty sqlite3 database
This step is important because refinery expects the database to already exist. The location of the database is mentioned in the refinery.toml config file. For now, the expected location is `/home/crazcalm/.config/rust-job-search/rust_job_search.sqlite3"`. 

Note: I have had trouble using `~` in the database 

`sqlite3 rust_job_search.sqlite3 "Vacuum;"`

Run migrations with `refinery `
