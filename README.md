### Euler Cryptor

RSA-implementation in Rust, provides a command line tool and a libary

#### Installation

```bash
cargo install --path .
```

#### Examples

##### Generating keys

```bash
$ euler-cryptor generate-key-pair --key-directory ./keys --key-pair-name mykeys
```

##### Encrypting file contents

```bash
$ cat file.txt | euler-cryptor encrypt --key-path ./keys/mykeys_pub.pem > encrypted_file.txt
```

or

```bash
$ euler-cryptor encrypt --key-path ./keys/mykeys_pub.pem --input file.txt --output encrypted_file.txt
```

##### Decrypting file contents

```bash
$ cat encrypted_file.txt | euler-cryptor decrypt --key-path ./keys/mykeys_sec.pem > file.txt
```

or

```bash
$ euler-cryptor decrypt --key-path ./keys/mykeys_sec.pem --input encrypted_file.txt --output file.txt
```


