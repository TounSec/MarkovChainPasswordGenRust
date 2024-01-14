# MarkovChainPasswordGenRust
MarkovChainPasswordGenRust is a password generator in Rust, which uses Markov chain theory to create random and complex passwords based on **Markov Chain** method. It offers a command line interface to specify password length `-l` `--length`. The generated password is then displayed in green for better visibility.

# Install
```
cd MarkovChainPasswordGenRust
cargo build
```

# Usage
```
🔢Generates a random password with Markov Chain method🔢

Usage: MarkovChainPasswordGenRust.exe [OPTIONS]

Options:
  -l, --length <LENGTH>  Sets length of the password [default: 16]
  -h, --help             Print help
  -V, --version          Print version
```

#### Specifying the length argument
```
.\MarkovChainPasswordGenRust.exe -l 32
Generated password : GW^c9mEqNK^09VGHbKQY97uoi5)Z0B^+
```

#### Without specifying the length argument
```
.\MarkovChainPasswordGenRust.exe
Generated password : r@MxPg8SIA)NlX*(
```
