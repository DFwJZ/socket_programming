# socket_programming

> Quick start

## No SSL or TLS implemented
1. Run `cargo build` to buid the program

2. Run `cargo run --bin server` on one terminal to start the server

3. Run `cargo run --bin client` on a separate terminal window to start client side 

4. Follow the instruction on the command line to continue the program.




## TLS implemented
### Generate your own private key and cert

1. Open a terminal window and navigate to a directory where you'd like to store your certificate and key files.
2. Generate a private key and a self-signed cetificate based on your private key using following line: <br>
`openssl req -newkey rsa:2048 -nodes -keyout key.pem -x509 -days 365 -out certificate.pem`
3. Combine your key and certificate in a PKCS#12 (P12) bundle: <br>
` openssl pkcs12 -inkey key.pem -in certificate.pem -export -out certificate.p12`
4. Add the following line to your Cargo.toml. <br>
    ```
    [dependencies]
    openssl = "0.10.45"
    lazy_static = "1.4.0"
    native-tls = "0.2.11"
    simplelog = "0.12.0"
    log = "0.4.17"
    time = "0.3.17"
    rpassword = "7.2.0"
    ```

5. Run `cargo build` to buid the program

6. Run `cargo run --bin tls_server` on one terminal to start the server

7. Run `cargo run --bin tls_client` on a separate terminal window to start client side 

8. Follow the instruction on the command line to continue the program. 




<br>
<br>
<br>

### Reference <br>
> 1. [How to install the Securly SSL certificate on Mac OSX ? ](https://support.securly.com/hc/en-us/articles/206058318-How-to-install-the-Securly-SSL-certificate-on-Mac-OSX-)
> 2. [Generating a self-signed certificate using OpenSSL](https://www.ibm.com/docs/en/api-connect/10.0.1.x?topic=overview-generating-self-signed-certificate-using-openssl)
> 3. dependencies reference: [native-tls](https://crates.io/crates/native-tls), [openssl](https://crates.io/crates/openssl), [lazy_static](https://crates.io/crates/lazy_static), [simplelog](https://crates.io/crates/simplelog), [log](https://crates.io/crates/log), [time](https://crates.io/crates/time), [rpassword](https://crates.io/crates/rpassword)