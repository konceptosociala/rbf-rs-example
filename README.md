# RBF/RS stack app example

_Yeah, I know that 3 different frontends for 3 different platforms in 3 different languages is crazy ;P_

It is a _simple_ cross-platform ToDo-application example, using rbf/rs stack:

* **Relm (Rust)** for desktop
* **Blazor (C#)** for web
* **Flutter (Dart)** for mobile
* **Rocket + SurrealDB (Rust)** for backend

## Build for desktop

```bash
$ cd todo_desktop
$ cargo run --release
```

## Build for mobile

```bash
$ cd todo_mobile
$ flutter build apk
```

## Build for web

```bash
$ cd todo_web
$ dotnet build TodoWeb.csproj --configuration Release
```