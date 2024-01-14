# MarkovChainPasswordGenRust
MarkovChainPasswordGenRust is a password generator with wordlist `-w` `--wordlist` in Rust, which uses Markov chain theory to create random passwords based on **Markov Chain** method. It offers a command line interface to specify password length `-l` `--length`. The generated password is then displayed in green for better visibility.

# Install
```
cd MarkovChainPasswordGenRust
cargo build
```

# Usage
```
🔢Generates a random password with wordlist and Markov Chain method🔢

Usage: MarkovChainPasswordGenRust.exe [OPTIONS] --wordlist <WORDLIST>

Options:
  -l, --length <LENGTH>      Sets length of the password [default: 16]
  -w, --wordlist <WORDLIST>  Specifies the path of the wordlist
  -h, --help                 Print help
  -V, --version              Print version
```

#### Specifying the length argument
```
.\MarkovChainPasswordGenRust.exe -w <WORDLIST> -l <LENGTH>
Generated password : GW^c9mEqNK^09VGHbKQY97uoi5)Z0B^+
```

#### Without specifying the length argument
```
.\MarkovChainPasswordGenRust.exe -w <WORDLIST>
Generated password : r@MxPg8SIA)NlX*(
```
