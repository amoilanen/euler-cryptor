### Euler Cryptor

RSA-implementation in Rust, provides a command line tool and a libary

#### Examples

##### Generating keys

```bash
$ euler-crypt generate-key-pair --key-directory ./keys --key-pair-name mykeys
```

##### Encrypting file contents

```bash
$ cat file.txt | euler-crypt encrypt --key-path ./keys/mykeys_pub.elr > encrypted_file.txt
```

##### Decrypting file contents

```bash
$ cat encrypted_file.txt | euler-crypt decrypt --key-path ./keys/mykeys_sec.elr > file.txt
```



