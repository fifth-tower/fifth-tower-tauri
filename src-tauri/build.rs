///  "build": {
///    "beforeDevCommand": "trunk serve",
///    "devUrl": "http://localhost:1420",
///    "beforeBuildCommand": "trunk build",
///    "frontendDist": "../dist"
///  },
///     "url": "http://project.5-tower.online"

fn main() {
    tauri_build::build()
}
