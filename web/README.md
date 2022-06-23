# 2022 Nutriblocks Kids Recipe App - Website

The app code repository for my 2022 NCEA assessment (& scholarship?) project.
Built for [Nutriblocks](https://nutriblocks.co.nz/).

## Layout

- `src/` - Contains routing, endpoint, and page code.
- `static/` - Static content and assets for the site.
- `tests/` - Playwright tests. If creating or changing the site, please add/change these tests appropriately.
- `.eslint*` - Linting config. For maintaining a consistent style.
- `.prettier*` - Code prettifier. Imagine linting but automatic and more forced.
- `package*` - Node/npm package config, mainly used for adding dependencies. Don't edit the lock file.
- `playwright.config.ts` - [Playwright](https://playwright.dev/) config file. [Read here](https://playwright.dev/docs/test-advanced) for more info.
- `svelte.config.js` - Svelte (and/or SvelteKit?) config file.
- `tsconfig.json` - Typescript config file.

&nbsp;

# How to Run the Code Locally

Install Node and npm. To check if it is installed, run the following

```
$ node -v
v16.15.1
$ npm -v
8.5.5
```

If you do not have them installed, download them from [here](https://nodejs.org/en/). I believe npm comes with the Node binary.

Now, from within this folder, run `npm i` to download all the npm packages. Use yarn or some other alternative if you want to for this step.

## Development

To launch the server, open the console and CD into this directory. Then simply run

```
$ npm run dev

> web@0.0.1 dev
> svelte-kit dev


  SvelteKit v1.0.0-next.350

  local:   http://localhost:3000
  network: not exposed
  network: not exposed
  network: not exposed

  Use --host to expose server to other devices on this network
```

Go to [`http://localhost:3000`](http://localhost:3000) and you will find the dev site. You may need to start the backend server.

## Production

> Todo
