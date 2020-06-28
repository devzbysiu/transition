<div align="center">

  <h1><code>transition</code></h1>

  <p>
    <strong>LED notification made easy</strong>
  </p>

  <p>
    <img src="https://github.com/devzbysiu/transition/workflows/Main/badge.svg" alt="Build status" />
    <a href="https://crates.io/crates/transition">
      <img src="https://img.shields.io/crates/v/transition?color=%2388C0D0&logoColor=%234C566A&style=flat-square" alt="Crates.io version" />
    </a>
    <a href="https://codecov.io/gh/devzbysiu/transition">
      <img src="https://img.shields.io/codecov/c/github/devzbysiu/transition?color=%2388C0D0&logoColor=%234C566A&style=flat-square&token=bfdc4b9d55534910ae48fba0b8e984d0" alt="Code coverage"/>
    </a>
    <a href="https://crates.io/crates/transition">
      <img src="https://img.shields.io/crates/l/transition?color=%2388C0D0&logoColor=%234C566A&style=flat-square" alt="License"/>
    </a>
    <a href="https://docs.rs/transition">
      <img src="https://img.shields.io/badge/docs-latest-blue.svg?color=%2388C0D0&logoColor=%234C566A&style=flat-square" alt="docs.rs docs" />
    </a>
  </p>

  <h4>
    <a href="#about">About</a>
    <span> | </span>
    <a href="#demo">Demo</a>
    <span> | </span>
    <a href="#installation">Installation</a>
    <span> | </span>
    <a href="#license">License</a>
    <span> | </span>
    <a href="#contribution">Contribution</a>
  </h3>

  <sub>Built with 🦀</sub>
</div>

# <p id="about">About</p>

This library allows you control the state of code execution using blink(1) LED notifier.

You simply:
1. Write code which you want to track:
  ```rust
  // example code
  println!("executing time consuming task");
  ```
2. Wrap your code with transition library:
  ```rust
  // start notification
  let notification = Transition::new(&["blue", "white"]).start()?;

  // our example code
  println!("executing time consuming task");

  // task finished with success
  notification.notify_success();
  ```

#### What does it do?
1. After calling `start()`, blink(1) starts blinking with blue and white colors interchangeably.
   This is done in a separate thread.
2. Then our code is executing.
3. At the end we call `notification.notify_success()` (or `notification.notify_failure()`) which
   changes the color of LED to green (or red). The colors can be changed:
   ```rust
   let transition = Transition::new(&["blue", "white"])
      .on_success("green")
      .on_failure("red");
   ```

# <p id="demo">Demo</p>

##  -- TODO

# <p id="installation">Installation</p>

Add
```toml
transition = "0.1.0"
```
to your `Cargo.toml`.

# <p id="license">License</p>

This project is licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

# <p id="contribution">Contribution</p>

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

