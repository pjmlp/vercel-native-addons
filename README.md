
# Node.js Hello World

Simple Node.js + Vercel example that shows how to have an API in Vercel, using nodejs addons written in C++, or serverless functions in TypesScript, Go and Rust, to work as a template for future projects.

## Overview

The API entry points are located under `api` folder, while the specific C++ addons are made available as local packages, under `packages`.

The mechanism to actually keep the native addons around after a deployment, without messing around with bundlers, trying to fit the output into Vercel's infrastructure, is to basically bundle them as proper packages.

For Go and Rust such trick isn't required, as they are directly supported by Vercel as official runtimes.

## How to Use

An example deploying into Vercel with its deployment CLI:

```bash
git clone https://github.dev/pjmlp/vercel-native-addons
```

Install the Vercel CLI:

```bash
npm i -g vercel
```

Then run the app at the root of the repository:

```bash
vercel dev
```

## How to test

An Open API configuration file is available ([openapi.yaml](openapi.yaml)), which can be imported into [Swagger Editor](https://editor.swagger.io/) for an easy to test the APIs, instead of calling them directly on the browser, or via curl.