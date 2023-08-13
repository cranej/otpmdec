A simple utility which decode Google Authenticator exported QR codes. By default it decode each exported otp item as format `issuer/name: secret` and print to stdout as a separate line.

For example, the following command scan Google Authenticator exported QR codes, decode it, and then encrypt the result:
```bash
zbarimg /path/to/qr_code.jpg | otpmdec | gpg -e -r 'recipient' -o /mnt/ramfs/otpsecrets.pgp
```

Alternatively, `-u/--uri` flag can be used to print in [Key Uri Format](https://github.com/google/google-authenticator/wiki/Key-Uri-Format) instead of the default simple format:
```bash
zbarimg /path/to/qr_code.jpg |otpmdec --url
```

Or if you are using `oathtool` directly, use `--secret-only` flag to print only base32 encoded secrets.

The Key Uri Format might be usefull for example if you are using [pass otp](https://github.com/tadfisher/pass-otp) like me.

The protobuf message defination used in this project was borrowed from [here](https://alexbakker.me/post/parsing-google-auth-export-qr-code.html).
