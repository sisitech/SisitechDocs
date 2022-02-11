
### Ubuntu 20.04
 Installing cerbot
```sh
sudo snap install --classic certbot
```

1. Preparing the cerbot command
```bash
sudo ln -s /snap/bin/certbot /usr/bin/certbot
```

1. Make sure port 80 is free. Stop nginx
```
sudo systemctl  stop nginx
```

1. Create a standalone certificate
```
sudo certbot certonly --standalone --preferred-challenges http -d example.com

```

##Result
``` txt linenums="1" hl_lines="2 3 4"
Successfully received certificate.
Certificate is saved at: /etc/letsencrypt/live/example.com/fullchain.pem
Key is saved at:         /etc/letsencrypt/live/example.com/privkey.pem
This certificate expires on 2022-05-11.
These files will be updated when the certificate renews.
Certbot has set up a scheduled task to automatically renew this certificate in the background.

- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
If you like Certbot, please consider supporting our work by:
 * Donating to ISRG / Let's Encrypt:   https://letsencrypt.org/donate
 * Donating to EFF:                    https://eff.org/donate-le
- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
```