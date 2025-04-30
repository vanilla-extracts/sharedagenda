# SharedAgenda CLI Client
This is the Rust crate of the CLI/REPL Client of SharedAgenda. 

## Install
A Makefile is provided.

### Dev 

```sh 
$ make
```

The binary will be located at `$PROJECT/target/debug/cli`

### Release

```sh 
$ make release
```

The binary will be located at `$PROJECT/target/release/cli`

## Configuration
The file where the config is for the database is at
`$HOME/.config/sharedagenda/cli.toml`

It looks like this:
```toml
api_link = "http://localhost:8008"
token = ""

[greeting]
greeting_message = "Welcome to SharedAgenda CLI REPL Version %version%, type help for help"
greeting_colour = "blue"

[prompt]
prompt_message = "> "
prompt_colour = "red"
```

You are not supposed to edit the `api_link` or `token` directly let the app
handle it. You can edit the rest of the config to change the greeting message
and colour. As well as the prompt message and colour.

Available colours are:
- "purple": Purple
- "cyan": Cyan
- "blue": Blue
- "black": Black
- "red": Red
- "yellow": Yellow
- "green": Green
- "white": White

In addition to those colours you can add a HTML colour hash like this `#55cdfc`
