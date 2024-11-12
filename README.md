# **term_tools: Rich API for Colorizing Terminal**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FSyaw0%2Fterm_tools%2Frefs%2Fheads%2Fdevelop%2FCargo.toml&query=package.version&label=Version&color=red)](https://crates.io/term_tools)

## **Overview**

term_tools is a Rust library that provides a rich API for colorizing terminal output. It allows you to create styled text strings with various colors, effects, and formatters.

## **Features**

- **Colors**: Supports 16 basic colors, 256 palette colors, and RGB colors.
- **Effects**: Supports slow blink and rapid blink.
- **Formatters**: Supports reset, bold, faint, italic, underline, and overline formatters.
- **Easy to use**: Simple and intuitive API for creating styled text strings.

## **Usage**

To use term_tools, simply add the following line to your `Cargo.toml` file:

```toml
[dependencies]
term_tools = "0.1.0"
```

Then, import the library in your Rust code:

```rust
use term_tools::styled;
```

Create a styled text string using the `styled` function:

```rust
let styled_text = styled("Hello, World!")
    .red()
    .bold()
    .underline()
    .paint();
println!("{}", styled_text);
```

## **Sequence of Styles**

The sequence of styles is important when using the `fg` and `bg` methods. These methods set the foreground and background colors, respectively, for all subsequent styles.

When you call `fg` or `bg`, all styles that come before it will be applied to the foreground or background, respectively.

Here's an example:

```rust
let styled_text = styled("Hello, World!")
    .red() // applies to foreground
    .fg() // sets foreground color to red
    .blue() // applies to background
    .bg() // sets background color to blue
    .paint();
```

In this example, the `red` style is applied to the foreground, and the `blue` style is applied to the background.

if there is only one call of `fg` or `bg` whole colors applied that `PaintType` for example:

```rust
let styled_text = styled("Hello, World!")
    .red() // red color
    .blue() // blue color
    .bg() // apply background color
    .magenta() // magenta color
    .paint();
```

in this example `paint` method will apply the background color of all colors.

if there is not any `fg` or `bg` call , the default paint type assume as `Foreground` for example:

```rust
let styled_text = styled("Hello, World!")
    .red() // red color
    .blue() // blue color
    .paint();
```

in this example the `paint` method will use foreground color of the colors.

## **Examples**

Here are some examples of using term_tools:

- **Basic colors**:

```rust
let styled_text = styled("Hello, World!")
    .red()
    .paint();
println!("{}", styled_text);
```

- **RGB colors**:

```rust
let styled_text = styled("Hello, World!")
    .rgb(255, 0, 0)
    .paint();
println!("{}", styled_text);
```

- **Effects**:

```rust
let styled_text = styled("Hello, World!")
    .bold()
    .underline()
    .paint();
println!("{}", styled_text);
```

- **Formatters**:

```rust
let styled_text = styled("Hello, World!")
    .reset()
    .bold()
    .paint();
println!("{}", styled_text);
```

## **License**

term_tools is licensed under the MIT License.

## **Contributing**

Contributions are welcome If you'd like to contribute to term_tools, please fork the repository and submit a pull request.

I hope this helps Let me know if you'd like me to make any changes.
