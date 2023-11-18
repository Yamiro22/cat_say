
```markdown
# Cat Say

Cat Say is a lightweight Rust program designed to elegantly display an ASCII art cat with customizable messages. Whether you want a playful feline greeting or a contemplative thought, Cat Say has you covered.

## Features

- **Customizable Messages:** Tailor the cat's speech to convey the desired message.
- **File-based Cat Pictures:** Optionally load a cat picture from a file for a personalized touch.
- **Expressive Visuals:** Toggle the cat's appearance, including a feature to make it appear with x-shaped eyes, adding a touch of humor or solemnity.

## Usage

```bash
# Build the Project
cargo build

# Run the Program
cargo run

# Run Tests
cargo test
```

## Command-line Options

- `-i, --stdin`: Read the message from STDIN instead of the command-line argument.
- `-f, --file <FILE>`: Load a cat picture from the specified file.
- `-d, --dead`: Make the cat appear with x-shaped eyes, suggesting a humorous or solemn expression.

## Examples

```bash
# Run with a Custom Message
cargo run -- "Hello, Cat!"

# Run with a Cat Picture from a File
cargo run -f cat_picture.txt

# Make the Cat Appear with x-shaped Eyes
cargo run -d
```

## Dependencies

- [clap](https://crates.io/crates/clap): A concise command-line argument parser.
- [colored](https://crates.io/crates/colored): Terminal color support to enhance visual appeal.
- [anyhow](https://crates.io/crates/anyhow): A robust error-handling library for concise and expressive error management.

## Contributing

Contributions are encouraged and welcomed! Please feel free to open issues or submit pull requests to enhance the project.

## License

This project is licensed under the [MIT License](LICENSE) - see the LICENSE file for detailed information.
```
