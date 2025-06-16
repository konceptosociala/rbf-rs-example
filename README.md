# Relm + Rocket + SurrealDB example

## Installation instructions

* Run SurrealDB database
    ```bash
    $ surreal start memory -A --user root --pass root --bind 0.0.0.0:8001
    ```

* Run API
    ```bash
    $ cd todo_api
    $ cargo run -- -A 127.0.0.1:8001 -U root -P root
    ```

* Run desktop app
    ```bash
    $ cd todo_desktop
    $ cargo run -- -A 127.0.0.1:8000/api -M http
    ```

## Screenshots

### Ubuntu
<img src="screenshots/ubuntu.png" alt="ubuntu" width=400/>

### Windows
<img src="screenshots/windows.png" alt="windows" width=400/>

### macOS
<img src="screenshots/macos.png" alt="macos" width=400/>

## License
This project is licensed under the Unlicense - see the [LICENSE](LICENSE) file for details.