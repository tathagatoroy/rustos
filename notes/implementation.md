* By default the linker in rust assumes a os and certain other details. For example for my system it assumes linux x64 and hence defaults to a C runtime. But we are building an OS hence we need a barebones target we assume no OS.  Such a bare metal environment is thumbv7em-none-eabihf target triple which describes an embedded ARM System
to set this is up run the following
    ```bash
    rustup target add thumbv7em-none-eabihf
    ```

    Now to build in this environment run 
    ``` 
    cargo build --target thumbv7em-none-eabihf
    ```