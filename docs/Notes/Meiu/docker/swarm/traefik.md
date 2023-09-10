




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
      - ovencrypt
      
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
    
    ports:
      - 80:80
        
      
    deploy:
      mode: global
      placement:
        constraints: 
          - node.role == manager      
      labels:
        - "traefik.enable=true"
        - "traefik.http.routers.traefikae.rule=Host(`traefik.example.com`)" 
        - "traefik.http.services.traefikae.loadbalancer.server.port=8080"
        - "traefik.docker.network=ovencrypt"
        - "traefik.http.middlewares.traefikae-auth.basicauth.users=micha:$$2y$$05$$NrR4hl3V7uCFT8nOdc5ZC.1AHuTjx4ysafhpBe2s0xX12eCG81VUO"
        - "traefik.http.routers.traefikae.middlewares=traefikae-auth"
          
  
  net:
    image: praqma/network-multitool
    networks:
      - ovencrypt
    
    deploy:
      mode: global
      
               
  nginxa:
    image: nginx
    
    networks:
      - ovencrypt
    
    deploy:
      mode: global
      placement:
        constraints: 
          - node.role == worker
    
      labels:
        - "traefik.enable=true"
        - "traefik.http.routers.nginxa.rule=Host(`example.com`)"   
        - "traefik.http.services.nginxa.loadbalancer.server.port=80"
        - "traefik.docker.network=ovencrypt"
        
  visualizer:
    image: dockersamples/visualizer
    
    
    networks:
      - ovencrypt
    
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
    
    deploy:
      mode: global
      placement:
        constraints: 
          - node.role == manager
          
      labels:
        - "traefik.enable=true"
        - "traefik.http.routers.visualizer.rule=Host(`swarm.example.com`)"   
        - "traefik.http.services.visualizer.loadbalancer.server.port=8080"
        - "traefik.docker.network=ovencrypt"
        - "traefik.http.routers.visualizer.middlewares=traefikae-auth"
   
volumes:
  agent-volume:
  
  portainer-data:
    
networks:
  ovencrypt:
    external: true
    attachable: true
    
  agent-network:
    
```

!!! note
    Confirm the `traefik.docker.network=` if routing not working as expected 

!!! note
    When deploying the service using `compose` the `labels` are attached to the `service` while for docker  `swarm`  the `labels` are attached to the `deploy` section.


## AUth

Generating basic auth
``` sh
sudo docker run --rm -i -t httpd:alpine htpasswd -nbB 'micha' '#dlajida8778^%&5da'
```