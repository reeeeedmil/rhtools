# rhtools
### a project made to standardize tools into a single package
#### These tools include IPv4 networking, filesystem managment and more

# Getting started
Download a binary for your OS from https://github.com/reeeeedmil/rhtools/releases (if it's supported) !!!This approach doesn't have newest changes!!!

If it's not supported/You want the newest stuff, follow the compilation guide:
## Compiling
1. I assume you already installed Rust. If not, follow the guide at https://rustup.rs/

2. Clone the repo with ```git clone https://github.com/reeeeedmil/rhtools```

3. Go into the directory using ```cd rhtools/```

4. Compile the project using ```cargo build --release```

5. Get your compiled app from the rhtools/target/release/ directory

## Usage
use the ```rhtools help``` command to show general help or ```rhtools <COMMAND> --help``` to show the help for a specific command

## Included tools

#### Copy
```rhtools copy -s <SOURCE DIRECTORY> -d <DESTINATION DIRECTORY>```

This command recursively copies the source directory and everything inside it to the destination directory.

#### Convert
```rhtools convert -b|-x|-o <NUMBER>```

Converts decimal number to binary, hexadecimal or octal.

#### Single subnet calculation
```rhtools net -a|--address <ADDRESS> -m|--mask (or -p|--prefix)```

Prints out all information for a network using inputted parameters

Both address and mask use XXX.XXX.XXX.XXX as their format.

#### Subnet scaffolding
```rhtools scaffold -a|--address <ADDRESS> -m|--mask (or -p|--prefix) --hosts <HOSTS> (or --prefixes <PREFIXES>```

Both address and mask use XXX.XXX.XXX.XXX as their format.

Hosts and prefixes use XX,XX,XX,XX as their format.
