# smm-zerop

This repo contains both the frontend and the backend for the Team 0% dashboard at [`smm-uncleared.com/`](https://smm-uncleared.com). Please check [the tools' public about page](https://smm-uncleared.com/about/) for details about what this is about.

## Contributing

If you want to report any issues, suggestions, or concerns, please reach out to `@denschub` on Discord. Likewise, if you want to contribute code, please ping `@denschub` before doing any work to make sure that no effort ends up wasted. :)

This project is, essentially, a fancy Rust application using `axum` and `sqlx`. To work on it, install the latest Rust toolchain. You need to set a bunch of config fields, and you probably want to do that via environmental variables. The provided `.env.example` should give you a good idea, `cargo run -- --help` will show you what's available.

The PostgreSQL database needs to exist, but the application will create all tables during startup. It will also dump in a few example levels so you have something to test against. The importer (`cargo run --bin importer`) would be there for you to get a full set of levels, but you most likely do not have access to the project's database, so that won't help you.

To get frontend resources during development, `cd frontend/`, `npm install`, and `npm start` in a second terminal. This will give you all the nicities you're used to, including live-reloading whenever you change CSS or JS.

## License

MIT.
