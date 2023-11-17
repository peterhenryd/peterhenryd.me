# peterhenryd-me

This repository contains the source code for [peterhenryd.me](https://peterhenryd.me), my personal website. The website is split
into two projects: the frontend and the backend.

## Stack

The frontend and the backend of the website are completely independent on one another on the source code level, and only
communicate through a REST API. This means that they each should run on their own process, and the backend does not
provide access to the frontend.

The frontend is written in TypeScript with Svelte, using TailwindCSS. The backend is written in Rust using Axum.

## License

This repository is currently unlicensed, meaning it is subject to copyright protections, and is public source rather 
than open source (as defined by the OSS Foundation). This decision is likely to change in the future.