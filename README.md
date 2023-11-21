# Gitee API Client

The Gitee API Client is a command-line tool that allows you to interact with the Gitee API. It provides similar functionality to the GitHub client, allowing you to perform various operations on your Gitee repositories.

## ToDo

- Authentication: Log in to your Gitee account using your username and password or personal access token.
- Repository Operations: Create, delete, and manage your Gitee repositories.
- Issue Management: Create, list, update, and close issues on your Gitee repositories.
- Pull Request Management: Create, list, update, and merge pull requests on your Gitee repositories.
- File Operations: Create, update, and delete files in your Gitee repositories.
- Branch Operations: Create, list, and delete branches in your Gitee repositories.
- Collaboration: Add, remove, and list collaborators on your Gitee repositories.

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/tolatolatop/rtgitee.git
   ```

2. Install the dependencies:

   ```bash
   cd rtgitee
   cargo build --release
   ```

3. Run the program:

   ```bash
   rtgitee [command] [options]
   ```

## Usage

To use the Gitee API Client, you need to provide your Gitee credentials or personal access token. You can pass them as command-line arguments or set them as environment variables.

```bash
# Using command-line arguments
rtgitee -u your-username -p your-password [command] [options]

# Using environment variables
export GITEE_USERNAME=your-username
export GITEE_PASSWORD=your-password
rtgitee [command] [options]
```

For detailed usage instructions and available commands, run the following command:

```bash
rtgitee --help
```

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).
