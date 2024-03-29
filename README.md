# Overview

**This repo is not intended to be `git cloned`**.

This repo is used by `dx create` when starting new projects. So by running `dx create` you are effectively running this code.

# Development

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload --platform desktop
```

# Building AppImages
On some platforms the environment currently needs to be set with `NO_STRIP=true` to build appimages properly.
Fix is from https://github.com/tauri-apps/tauri/issues/8929.
