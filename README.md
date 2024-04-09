# Example Dioxus app

This is Project I of CS372 done in Rust via Dioxus Fullstack and TailwindCSS.

## Running

### Tailwind

If you want to see styling applied, you'll need to install `tailwindcss` via NPM.

Install (NodeJS)[https://nodejs.org/en] to get `npm`.

Install TailwindCSS:
```console
npm install -D tailwindcss
```

`cd` to the repo's directory and run TailwindCSS:
```console
npx tailwindcss -i ./src/input.css -o ./src/output.css --watch
```

### Dioxus

To build the web application locally, you need to install Rust and Cargo. It's suggested to use Rust through [Rustup](https://rustup.rs/).

You'll also need the Dioxus CLI tool to build the app, which can be installed by running:

```bash
cargo install dioxus-cli
```

Finally, run the application in the project directory using `dx serve` which should return the result:

```bash
Dioxus @ vX.X.X [...]

        > Local : http://localhost:8080/
        > Network : http://.../
        > HTTPS : Disabled

        ...

        > Build Time Use : ... millis

[INFO] A perfect compilation!
```

You can now open `http://localhost:8080/` and use the application locally.

## Contributing

### Git Hooks

To use Git Hooks properly, run the following command in the root of the repository:

```bash
pip install pre-commit
pre-commit install
```
