# iskissingthehomiesgoodnight.gay
*The premier online educational resource on homie kissing*

## What

Like so many things, this project started with those small yet powerful words; "Wouldn't it be funny if..."

This project aims to serve as an open design project, building a style library to ensure that learning about whether kissing your homies goodnight is gay is not only educational, but also **fun**. As a secondary purpose, I hope this project can be a place where budding web developers can dip their toes into the wide world of open-source development by only needing some CSS and a few lines of JSON to get started contributing.

## How

The idea behind this project is to create some basic dynamic web content without forcing the client to perform additional work generating or fetching said content, using a basic form of Server Side Rendering.
The end result is a two-part project consisting of a web server written in Rust and a simple templating system. When someone accesses the website, the server looks at the library of available styles and picks one at random to inject into the HTML template before serving it back to the client. This way no code needs to be executed by the client, and no additional network requests need to be made to deliver stylesheets or optional JavaScript.

## Contributing

To get started, you probably want to be able to run the development server to be able to preview your contribution!

### Installing dependencies

Currently, building this project requires

- Git, to be able to clone this repository
- Curl, to be able to install some other dependencies
- Node 16 and npm, 
- A Rust compiler and Cargo build system
- A linker (cc)
- GNU Make
- (Optional) openssl-dev to build the server with HTTPS support

The following steps should allow you to install them

1. Install the most readily available dependencies from your package manager of choice \
  Ubuntu (and other Debian derivatives): `sudo apt install git curl gcc make` \
  Fedora: `sudo dnf install git curl gcc make`

2. Install the Rust build system \
  See https://rustup.rs/ for the recommended install method

3. Install Node 16 \
  I highly recommend doing this by installing nvm (see: https://github.com/nvm-sh/nvm), \
  then installing Node 16 with `nvm install 16 && nvm use 16`

### Building

First, clone this repository using `git clone https://github.com/jakobkg/iskissingthehomiesgoodnight.gay`, and move into the cloned repository with `cd iskissingthehomiesgoodnight.gay`. Now, simply run the build script by running `make`, and the project will build! The first time building might take a while, as you're going to be fetching and building a lot of dependencies from NPM and Cargo. After the first build process, these will be reused and you should be able to complete the process in a few seconds!

Finally, you can now run the server by running `(cd build; ./server)`! You should see the server starting in your terminal, and can see the site by opening https://localhost:8080 in a web browser!

### Getting started

TODO: implement specific style viewing in dev server and refer to it here

The most basic contribution to this project consists of two parts:
- A .css file with styling for the website, placed in `src/styles/<your username>/<name of the style>.css`.
  - This CSS will be automatically applied to the HTML by the server.
- An entry in `contributors.json`, containing your GitHub username and the name of the stylesheet you added. 
  - This file is used by the server to find stylesheets and scripts to inject into the HTML before serving it to a client.

That's it!

If you're feeling fancy, you may also add a script file such as `src/scripts/<your username>/<name of the style>.js` to add things such as animations to your contribution.

### Guidelines

1. You may not edit or affect other contributors' styles. A typical contribution should only touch `src/styles/<your username>/`, `src/scripts/<your username>/` and `contributors.json`.
2. The word "No" should be clearly readable on the page when your style and/or script is applied.
3. Feel free to add additional build tools or technologies such as SASS or SCSS to the `npm build` action, as long as build times remain reasonable!

For a sample pull request, [see #6](https://github.com/jakobkg/iskissingthehomiesgoodnight.gay/pull/6)
