# How to run:

## Manually
1. Clone this repo
2. Create `.env` file based on the `.env.example`
3. Run database with docker compose postgres up -d
4. Build the project with `cargo build`
5. Run migrations with sea-orm-cli - `sea-orm-cli migrate up`
6. Run the app - `cargo run` (add `--release`, if you want to run it in release mode)

## Less manually
1. Clone this repo
2. Create `.env` file based on the `.env.example`
3. Run `docker compose up` to start of db and backend.
4. Use `sea-orm-cli migrate up -u $DATABASE_URL`, to trigger all migrations from your machine.

> But this less manuall version is not tested

# Todo:
- [ ] automate the database migration process (`sea-orm-cli migrate up`).
- [ ] filter car logs by the time the car arrived.
- [ ] tests
