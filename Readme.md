A simple utility which decode Google Authenticator exported QR codes.

```bash
# otpmdec --help
Decode Google Authenticator migration data as "issuer/name: secret" lines to stdout.
  Read input from stdin - you can either manually type/paste or pipe to it.
```

For example, the following command scan Google Authenticator exported QR codes, decode it, and then encrypt the result:
```bash
zbarimg /path/to/qr_code.jpg | otpmdec | gpg -e -r 'recipient' -o /mnt/ramfs/otpsecrets.pgp
```

The protobuf message defination used in this project was borrowed from [here](https://alexbakker.me/post/parsing-google-auth-export-qr-code.html).
