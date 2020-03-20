Catsay Command line Application in rust

# Run
- cargo run

# Details
catsay 0.1.0
dop3ch3f <ifeanyi.ibekie@gmail.com>

USAGE:
    catsay.exe [FLAGS] [OPTIONS] [message]

FLAGS:
    -d, --dead       Make the cat appear dead
    -h, --help       Prints help information
    -i, --stdin      Read the message from STDIN instead of the argument    
    -V, --version    Prints version information

OPTIONS:
    -f, --file <catfile>    Load the cat picture from the specified file    

ARGS:
    <message>    What does the cat say? [default: Meow!]


# Features
- A command-line program in Rust. 
- Read simple command- line arguments.
- Parse more complex arguments.
- Positional arguments, binary flags, and options,description and default values to them.
- Coloring, reading from a file, and accepting standard input and output to standard output and standard error. 
- Integration tests
