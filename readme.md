## About Me
**Christian Pavilonis**

I work [@Timbergrove](https://timbergrove.com/), and am currently building a tauri app targeting windows for tracking power tools over bluetooth.

GitHub: [@ChristianPavilonis](https://github.com/ChristianPavilonis)

X: [@ChristianThePav](https://twitter.com/ChristianThePav)
## Intro
Simply put, Tauri is a framework for building rust powered desktop apps with a web frontend. Like Electron but way better.

### How it works
Unlike Electron which essentially requires a full Chromium installation for the app, Tauri uses native web views (OS level). This allows for tiny binaries.

## Install
[Prereqs](https://tauri.app/v1/guides/getting-started/prerequisites) | [Quick Start](https://tauri.app/v1/guides/getting-started/setup/)

Tauri provides tooling to get started through cargo or your javascript package manager of choice.

```sh
cargo create-tauri-app
# or
<npm|yarn|pnpm|bun> create tauri-app # tauri-app@latest for npm
```

Follow the prompts, choose your stack, then cd in and get started.

#### Basic project structure.

```
- my-tauri-app
    - src # Frontend src
    - src-tauri # Rust src
        - tauri.conf.json # where you can configure tauri specific things
```

[**tauri.conf.json**](https://tauri.app/v1/api/config)
- default window size
- cli config (get to that later)
- bundling

#### Start development
```
cargo tauri dev
# or use script in package.json with npm, etc.
```

#### Invoking Rust from the frontend

First you need a tauri::command
```rust
#[tauri::command]
fn my_command() -> <anything that can be Serialized> {
    ...
}
```

Add the command to the tauri::generate_handler! macro.

```js
await invoke("my_command", {...});
```


## System tray apps
[Guide](https://tauri.app/v1/guides/features/system-tray)

### Native menus
You can make a whole tauri app that lives in the system tray without writing any html.

- Instantiate a `SystemTray`
- call `.system_tray()` on the `tauri::Builder`
- handle events with `.on_system_tray_event()` closure.

### Custom System tray UI
Create a pretty little tray app with web tech.

- Instantiate a `SystemTray` and leave it empty.
- call `.system_tray()` on the builder
- handle events with `.on_system_tray_event()` closure.
    - Create a window using `tauri::WindowBuilder`, or hide/show.
        - Set the size
        - decorations to false (remove chrome)
        - always on top to true
    - Use a [plugin](https://github.com/tauri-apps/tauri-plugin-positioner) to position the window.

#### Notes on plugins
They often come with both a rust crate and an npm package. (some have to be installed via github url)
Register by calling `.plugin()` with the `<plugin>::init()` function in the plugin's module


## CLIs
[Guide](https://tauri.app/v1/guides/features/cli)

Tauri supports including a CLI with your app using the clap crate under the hood.

### Defining commands/arguments
To define a cli add a cli block to the `tauri.conf.json` file.
```
{
  "tauri": {
    "cli": {
      "description": "", // command description that's shown on help
      "longDescription": "", // command long description that's shown on help
      "beforeHelp": "", // content to show before the help text
      "afterHelp": "", // content to show after the help text
      "args": [], // list of arguments of the command, we'll explain it later
      "subcommands": {
        "subcommand-name": {
          // configures a subcommand that is accessible
          // with `./app subcommand-name --arg1 --arg2 --etc`
          // configuration as above, with "description", "args", etc.
        }
      }
    }
  }
}
```

### Handle CLI matches
```rust
match app.get_cli_matches() {
    ...
}
```

### State Management
[Guide](https://tauri.app/v1/guides/features/command/#accessing-managed-state)
- Define a struct
- call `app.manage(my_struct)`
- inside of command use `tauri::State<MyStruct>`

## Tauri 2.0
Tauri 2.0, currently in alpha, supports IOS and android, as well as Swift and Kotlin bindings.

#### Resources
[Awesome Tauri](https://github.com/tauri-apps/awesome-tauri) OSS apps, plugins

[My Chat GPT client](https://github.com/ChristianPavilonis/chat-gpt)

[Amish Commit](https://github.com/amishdev/amish-commit)


