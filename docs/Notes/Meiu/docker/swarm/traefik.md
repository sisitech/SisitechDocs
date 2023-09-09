




Example

```yml
version: "3.9"

services:
  traefika:
    image: traefik:v2.4
    command:
      - --log.level=DEBUG
      - --entrypoints.httpa.address=:80
      - --providers.docker=true
      - --providers.docker.exposedByDefault=true
      - --providers.docker.swarmMode=true
      - --api=true
      - --api.dashboard=true
      - --api.insecure=true
      - --accesslog=true
      - --providers.docker.watch=true
      
    networks:
      - traefik-public
      
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
    
    ports:
      - 80:80
      - 8081:8080
        
      
    deploy:
      mode: global
      placement:
        constraints: 
          - node.role == manager      
      labels:
        - "traefik.enable=true"
        - "traefik.http.routers.traefikae.rule=Host(`traefik.example.com`)" 
        - "traefik.http.services.traefikae.loadbalancer.server.port=8080"
        - "traefik.docker.network=traefik-public"
          
  nginxa:
    image: nginx
    
    networks:
      - traefik-public
    
    deploy:
      mode: global
      placement:
        constraints: 
          - node.role == manager
    
      labels:
        - "traefik.enable=true"
        - "traefik.http.routers.nginxa.rule=Host(`example.com`)"   
        - "traefik.http.services.nginxa.loadbalancer.server.port=80"
        - "traefik.docker.network=traefik-public"
      
    
networks:
  traefik-public:
    external: true

```

!!! note
    Confirm the `traefik.docker.network=` if routing not working as expected 

!!! note
    When deploying the service using `compose` the `labels` are attached to the `service` while for docker  `swarm`  the `labels` are attached to the `deploy` section.