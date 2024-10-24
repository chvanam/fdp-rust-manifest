# rustui - a fully typed safe NETB/MQTT bridge

## What it is?

`rustui` is a Rust static library that is linked to NETB's `ui` target using [Corrosion]().

Corrosion is a CMake extension that ...

In any `CMakeLists.txt` To add Corrosion :

```CMake
include(FetchContent)
FetchContent_Declare(Corrosion GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git GIT_TAG v0.4.7)
FetchContent_MakeAvailable(Corrosion)
```

You can then create a target that will have the same name as the Rust package, here `rustui`.

```
corrosion_import_crate(MANIFEST_PATH rustui/Cargo.toml)

esmi_module(${netb_exe}
  NAME "ui"
  GROUP "ui"
  SOURCES ${sources}
  DEPENDS ${ui_deps} rustui
  EXTERNAL_DEPENDS ${ext_deps})
```

## 