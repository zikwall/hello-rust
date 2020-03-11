<div align="center">
  <h1>Hello Rust</h1>
  <h4>Begin!</h4>
  <h5>First aka Hello Rust experience</h5>
</div>

### Rust not installed?

1. `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. `export PATH=$HOME/.cargo/bin:$PATH` or add `bashrc`, `*_profile`
3. `source ~/.bashrc`

#### Not help? This still is broken in Linux.

- [x] when you edit the .profile you should be putting.

```shell script

if [ -d "$HOME/.cargo/bin" ] ; then
	PATH="$HOME/.cargo/bin:$PATH"
fi

```
- [x] You should run the following as the very last thing the main() function does.

```shell script

$(source ~/.profile)    
# or 
# exec source $HOME/.profile
exit

```

3. `rustc --version` => `rustc 1.41.0 (5e1a79984 2020-01-27)`

### Run Hello

1. `rustc hello.rs --out-dir builds/`
2. `cd builds/ && ./hello`
3. Happy ^_^
4. But this is not serious, soon it will be so...

### Run Hello Actix

1. `cargo run`
2. visit `localhost:8088` and `localhost:8088/again`
3. `cargo build --target-dir ../builds/ --release`
4. `cd builds/release  && ./hello_actix`

### Guess Game

1. `cd guessing_game/ && cargo run`
2. Good luck

### How IDE use?

You can use the IntelliJ family IDEs such as PyHarm, GoLand, PHPStorm & etc. 
Just install the appropriate plugin for Rust, and you can also install a special color plugin:

IntelliJ Rust: `https://intellij-rust.github.io/docs/quick-start.html`