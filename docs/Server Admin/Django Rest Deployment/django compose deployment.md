# Django Static /  Media Files File System Storage

## Compose config
```yml
version: "3.8"
services:
  media:
    image: nginx:1.15
    networks:
      - db
    ports:
      - 8888:80

    volumes:
      - media:/usr/share/nginx/media
      - static:/usr/share/nginx/static
      - ./nginx.conf:/etc/nginx/conf.d/default.conf

```

## NGINX config
```config
server {
  listen 80;
  location /static {
    alias /usr/share/nginx/static;
  }
  location /media {
    alias /usr/share/nginx/media;
  }
}
```

## API Config

```yml
  api:
    image: michameiu/somapi:v1.0.6.arm # Location with a Dockerfile
    restart: always
    depends_on:
      - db
    networks:
      - db
    ports:
      - 8000:8000
    volumes:
      - media:/media
      - static:/static
    environment:
      - SECRET_KEY=testdummy
      - DB_HOST=db
      - MEDIA_URL=http://localhost:8888/media/
      - STATIC_URL=http://localhost:8888/static/
```

