# A simple generator for OpenAI Fine Tuning

This project is a simple project that should make easier to generate JSONL file for the OpenAI Fine Tuning API.

## Usage

For the learning, you have to build a directory with the following structure:

```
.Fine Tuning Project
├── Learn1
│   ├── prompt.md
│   └── completion.md
├── Learn2
│   ├── prompt.md
│   └── completion.md
└── Learn3
    ├── prompt.md
    └── completion.md
```

This way, you can separate the prompt with the expected completion.

Then, you can run the following command:

```shell
chatter --path <Fine Tuning Project Directory> --output <Output file>
```

## Options

### Code block
`-c`: Use a code block for the prompt

### Prompt modifier
`-m`: Add a modifier to the prompt

The prompt modifier is something that will be added to the prompt. For example, if you want to add a modifier to the prompt, you can use the following command:

```shell
chatter --path <Fine Tuning Project Directory> --output <Output file> --prompt-modifier "This is a modifier"
```

This will make possible to add a modifier that will be common to all the prompts.

### Help
```
Usage: chatter [OPTIONS] --path <PATH> --output <OUTPUT>

Options:
  -p, --path <PATH>
  -o, --output <OUTPUT>
  -m, --prompt-modifier <PROMPT_MODIFIER>
  -c, --code
  -h, --help                               Print help
  -V, --version                            Print version
```
