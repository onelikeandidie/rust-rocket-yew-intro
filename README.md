# Meetballs Introduction to Rust's Rocket and Yew crates

This repository is a write up of a session of a Meetballs programming meetup.

## Dependencies

Install [Rust](https://rustlang.org) and add the WebAssembly compilation target.

```bash
# Rust install script
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# WebAssembly target
rustup target add wasm32-unknown-unknown
```

To make [Yew.rs](https://yew.rs) easier to use, we also need to install Trunk,
a bundling tool for JavaScript and WebAssembly. This step might take a bit as
it is compiled by running this command, some distributions have pre-built
binaries. Check [Trunkrs.dev](https://trunkrs.dev) for more info.

```bash
cargo install --locked trunk
```

Also in order to use Tailwind, nodejs must be installed and the `npx` command
must be available. To make sure you can use the `npx` command run:

```bash
npx tailwindcss
# The output should be the cli's help doc
```

# Part 1 - Project outline and initial setup

The project we are doing today is a simple todo app that has both a frontend
and backend fully written in Rust.

The idea is simple, the end user will be able to edit a list with tasks todo
and save those tasks to the server so that when they re-open the page they can
see their tasks again.

The frontend will consist of a stateful Yew.rs ui using Tailwind for styling
that will send requests to the Rocket.rs backend to add, edit and delete tasks.

## Setup 

To integrate all of the parts of this project, we will use a Cargo Workspace.

_TODO: Add info on Cargo Workspaces_

To create a workspace, you'll have to create a Cargo.toml file on the root of
the project with the `[workspace]` directive then with a list of components
that are part of this project, like bellow:

```toml
[workspace]
members = [
    "frontend", // Our Yew frontend
    "task-core", // Shared structs (more on that later)
    "backend" // Out Rocket backend
]
```

After setting this up we can now use `cargo` to create our crates! Change into
the project directory and run the following commands to auto-generate our hello
worlds.

```bash
cargo new frontend
cargo new backend
cargo new task-core --lib # This ensures that the created crate is a library
```

We are now done setting up our project.

# Part 2 - Simple Counter Frontend

Before we work on the frontend, we'll have to add the required dependencies,
the dependencies required to create the base for our frontend are `yew` and
`web-sys` to the `frontend/Cargo.toml`:

```toml
# ...
[dependencies]
yew = { version = "0.21.0", features = ["csr"] }
web-sys = "0.3.66"
```

To ensure that we keep parity of the Task struct for our project we'll also
include `task-core` as a local dependency.

```toml
# ...
[dependencies]
yew = { version = "0.21.0", features = ["csr"] }
web-sys = "0.3.66"
task-core = { path = "../task-core" }
```

The next thing is to create `frontend/index.html` outside of the `src` with the
following contents:

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1"/>
    <title>Meetballs Yew + Rocket</title>
</head>
<body>
</body>
</html>
```

To make styles a little quicker to iterate, the example uses `tailwind` to
automagically make styles for the webpage so we add that to the html head
tag.

```html
<!-- ... -->
<head>
    <!-- ... -->
    <link data-trunk rel="tailwind-css" href="src/tailwind.css"/>
    <base data-trunk-public-url/>
    <!-- ... -->
</head>
<!-- ... -->
```

And add a `frontend/tailwind.config.js` with the following configuration to
support rust content:

```js
module.exports = {
    mode: "jit",
    content: {
        files: ["src/**/*.rs", "index.html"],
    },
    darkMode: "media", // 'media' or 'class'
    theme: {
        extend: {},
    },
    variants: {
        extend: {},
    },
    plugins: [],
};
```

And also a `frontend/src/tailwind.css` with the base content:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

Then we add the example code to the `frontend/src/main.rs` from the Yew.rs
tutorial to get a simple counter!

```rust
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div class="m-4">
            <button class="px-2 py-1 border rounded border-neutral-600" {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

Finally, opening `frontend` in a terminal we can run `trunk serve` to see our
result. If your port 8080 is busy, you can pass the `--port 8000` argument to
change the output port.

# Part 3 - Do todo

# Part 4 - Hello Rocket.rs

# Part 5 - Implement API

CORS AHHHHHH

# Part 6 - Intertwine Backend and Frontend