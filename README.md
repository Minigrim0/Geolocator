# Geolocator 🌍
A tauri app to help localise your pictures.

![application screenshot](.github/screenshot/.png)

# Usage 🧰
Start the application (see [building](#building) for details on how to compile), click on the `load folder` button, select the Photo folder you want to locate.
The application will then load every picture that can store EXIF metadata, and try to read the current metadata from it. The pictures will then be displayed in the
top right corner of the application, allowing you to click a point on the map, and then confirm to save the geolocation to the picture's metadata.

# Building 🏗
Install rust on your machine (see [rustup](https://rustup.rs) for an easy install) and the tauri-cli dependency `cargo install tauri-cli`.
Then, from the root of the project type `cargo tauri build` to build the project in release mode. This might also try to build bundles,
which may lead to errors being displayed in the console. You can safely ignore these if you only need the executable. The executable will be
located under `src-tauri/target/release/geolocator`.
