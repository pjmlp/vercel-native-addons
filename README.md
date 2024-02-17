
# Node.js Hello World

Simple Node.js + Vercel example that shows how to have an API in Vercel, using addons written in C++ and Rust, to work as a template for future projects.

## Overview

The API entry points are located under `api` folder, while the specific C++ and Rust addons are made available as local packages, under `packages`.

The mechanism to actually keep the native addons around after a deployment, without messing around with bundlers, trying to fit the output into Vercel's infrastructure, is to basically bundle them as proper packages.


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