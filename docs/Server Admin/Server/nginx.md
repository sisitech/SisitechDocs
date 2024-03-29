# Nginx  Ubuntu 20.04

## Installing

``` bash
sudo apt-get update
sudo apt-get install nginx
```

----

## Configuring Django Rest API

- Create an api config file inside

``` bash

cd /etc/nginx/sites-available
nano sampeApi
```

- Paste the following sample and replace the higlighted lines

```config linenums="1" hl_lines="7 12 13 17 22" title="sampleApi"
map $http_upgrade $connection_upgrade {
        default upgrade;
        '' close;
    }

server {
    server_name api.pl-emis.com;

    client_max_body_size 75M;   # adjust to taste
    #listen 80;
    listen 443 ssl; # managed by Certbot
    ssl_certificate /etc/letsencrypt/live/api.pl-emis.com/fullchain.pem; # managed by Certbot
    ssl_certificate_key /etc/letsencrypt/live/api.pl-emis.com/privkey.pem; # managed by Certbot
    # Django media

    location /media  {
        alias /home/azureuser/apps/api/media;
    }


    location /static {
        alias  /home/azureuser/apps/api/static;
    }

 
    location / {
        #include proxy_params;
        proxy_pass http://127.0.0.1:8000;

        proxy_read_timeout 10m;
        proxy_connect_timeout 10m;
        proxy_send_timeout 10m;
        #proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        #proxy_set_header Host $host;  # pass the host header - http://wiki.nginx.org/HttpProxyModule#proxy_pass

        proxy_http_version 1.1;  # recommended with keepalive connections - http://nginx.org/en/docs/http/ngx_http_proxy_module.html#proxy_http_version

        # WebSocket proxying - from http://nginx.org/en/docs/http/websocket.html
        #proxy_set_header Upgrade $http_upgrade;
        #proxy_set_header Connection $connection_upgrade;
        proxy_set_header    X-Forwarded-SSL on;
        proxy_redirect off;
        proxy_set_header X-Forwarded-Proto https;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Host $server_name;

    }

}
```

## Configuring letsEncrypt SSL

Refer to [Setting Up Letsencrypt](../letsencrypt) on installing and requesting for a certificate using letsencrypt

If you do not have an ssl, uncomment `#listen 80;` and comment `listen 443 ssl;` statements together with the certifcate paths as shown belows

``` linenums="5" title="sampleApi"
listen 80;
#listen 443 ssl;
#ssl_certificate /etc/letsencrypt/live/api.pl-emis.com/fullchain.pem; # managed by Certbot
#ssl_certificate_key /etc/letsencrypt/live/api.pl-emis.com/privkey.pem; # managed by Certbot
```

## Redirect http -> https

Add the following lines and replace the server_name

``` linenums="1" hl_lines="4"
server {
    listen 80 ;

    server_name server_name;

    return 301 https://$host$request_uri;
}
```

## Configuring A static angular App
```config
server {
        listen 80;
        listen 443 ssl;
        ssl_certificate /etc/letsencrypt/live/onekana.naconek.ke/fullchain.pem; # managed by Certbot
        ssl_certificate_key /etc/letsencrypt/live/onekana.naconek.ke/privkey.pem; # managed by Certbot

        include /etc/nginx/mime.types;
        server_name onekana.naconek.ke;
        root /home/daa/apps/dash;
        location / {
                try_files $uri$args $uri$args/ /index.html;
        }
        location ~ \.css {
                add_header  Content-Type    text/css;
        }
        location ~ \.js {
                add_header  Content-Type    application/x-javascript;
        }
}
```

## Enable API Config

- Create a symbolic link of the config in the `sites-enabled` folder

```
sudo ln -s /full/path/sites-available/sampleApi /full/path/sites-enabled/sampleApi
```

!!! caution
    It must be the absolute path not a relative path

----

## Configuring Static Dashboard

- Create an api config file inside

```
cd /etc/nginx/sites-available
nano sampeDash
```

- Paste the following sample and replace the higlighted lines

``` sh title="sampeDash" linenums="1" hl_lines="3 5 6 19"


   server {
        #listen 80;
        server_name onekana.naconek.ke;
        listen 443 ssl;
        ssl_certificate /etc/letsencrypt/live/api.onekana.naconek.ke/fullchain.pem; # managed by Certbot
        ssl_certificate_key /etc/letsencrypt/live/api.onekana.naconek.ke/privkey.pem; # managed by Certbot

        root  /home/daa/apps/dashboard;
        include /etc/nginx/mime.types;

        location / {
            try_files $uri /index.html;
        }
    }

server {
    listen 80 ;

    server_name onekana.naconek.ke;

    return 301 https://$host$request_uri;
}

```

---

## Websockets

Make sure the highlighted sections are not commented.

```ini linenums="1" hl_lines="1 2 3 4 39 40" title="sampleApi"
map $http_upgrade $connection_upgrade {
        default upgrade;
        '' close;
    }

server {
    server_name api.pl-emis.com;

    client_max_body_size 75M;   # adjust to taste
    #listen 80;
    listen 443 ssl; # managed by Certbot
    ssl_certificate /etc/letsencrypt/live/api.pl-emis.com/fullchain.pem; # managed by Certbot
    ssl_certificate_key /etc/letsencrypt/live/api.pl-emis.com/privkey.pem; # managed by Certbot
    # Django media

    location /media  {
        alias /home/azureuser/apps/api/media;
    }


    location /static {
        alias  /home/azureuser/apps/api/static;
    }

 
    location / {
        #include proxy_params;
        proxy_pass http://127.0.0.1:8000;

        proxy_read_timeout 10m;
        proxy_connect_timeout 10m;
        proxy_send_timeout 10m;
        #proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        #proxy_set_header Host $host;  # pass the host header - http://wiki.nginx.org/HttpProxyModule#proxy_pass

        proxy_http_version 1.1;  # recommended with keepalive connections - http://nginx.org/en/docs/http/ngx_http_proxy_module.html#proxy_http_version

        # WebSocket proxying - from http://nginx.org/en/docs/http/websocket.html
        #proxy_set_header Upgrade $http_upgrade;
        #proxy_set_header Connection $connection_upgrade;
        proxy_set_header    X-Forwarded-SSL on;
        proxy_redirect off;
        proxy_set_header X-Forwarded-Proto https;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Host $server_name;

    }

}
```

## Testing & Restart

- Test nginx config

```
sudo nginx -t
```

- Restart nginx

```
sudo systemctl restart nginx
```
