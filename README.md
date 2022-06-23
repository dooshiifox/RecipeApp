# 2022 Nutriblocks Kids Recipe App - Server

The code repository for my 2022 NCEA assessment (& scholarship?) project.
Built for [Nutriblocks](https://nutriblocks.co.nz/).

## App

The app is built with Flutter due to its fast development times, hot reloading, and ability to compile to web, iOS, and Android for a consistent experience.

The app source is located in `app/`, as well as a README with more information.

## Web

Despite using Flutter, a completely different UI would be better for larger screens that isn't focused around making mobile apps, hence a different tool. SvelteKit + Svelte + TypeScript is used for the website version of the code.

The wbe source is located in `web/`, as well as a README with more information.

## Server

The server is the backend and database of the app. It's built with Actix-Web because it's fast and I feel like a Rust backend is the most safe.

The server source is located in `server/`, as well as a README with more information.

## Layout

-   `.vscode/` - VS Code config files. Contains build and test tasks.
-   `app/` - The app source code. `app/README.md` has more information on the folder's contents and running the app.
-   `server/` - The server source code. `server/README.md` has more information on the folder's contents and starting the server.
-   `web/` - The website source code. `web/README.md` has more information on the folder's contents and starting the website.
-   `LICENSE` - Usual license info. Uses the Apache 2.0 license.
-   `README.md` - :)
