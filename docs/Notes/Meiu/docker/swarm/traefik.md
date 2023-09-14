




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


## Traefik With SSL

```yml title="bash"
version: "3.9"

services:
  process_tasks:
    image: michameiu/moekeapi:v2
    deploy:
      replicas: 3
    networks:
      - ovencrypt
    command: python manage.py process_tasks

  api:
    image: michameiu/moekeapi:v2
    
    networks:
      - ovencrypt
    
    deploy:
      replicas: 2
      labels:
        - "traefik.enable=true"
        - "traefik.http.routers.api.rule=Host(`api.micha.africa`)" 
        - "traefik.http.services.api.loadbalancer.server.port=8000"
        - "traefik.http.routers.api.tls=true"
        - "traefik.http.routers.api.tls.certresolver=letsencrypt"
        - "traefik.docker.network=ovencrypt"
        
  traefik:
    image: traefik:latest
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
      # Letsencrypt setup
      
      - --certificatesresolvers.letsencrypt.acme.dnschallenge=true
      - --certificatesresolvers.letsencrypt.acme.dnschallenge.provider=cloudflare
      - --certificatesresolvers.le.acme.dnschallenge.resolvers=1.1.1.1:53,8.8.8.8:53
      - --certificatesresolvers.letsencrypt.acme.email=michameiu@gmail.com
      - --certificatesresolvers.letsencrypt.acme.storage=/letsencrypt/acme.json
      # Set up an insecure listener that redirects all traffic to TLS
      - --entrypoints.websecure.address=:443      
      - --entrypoints.httpa.http.redirections.entrypoint.to=websecure
      - --entrypoints.httpa.http.redirections.entrypoint.scheme=https
      # Set up the TLS configuration for our websecure listener
      - --entrypoints.websecure.http.tls=true
      - --entrypoints.websecure.http.tls.certResolver=letsencrypt
      
     
      
    
    environment:
      - CLOUDFLARE_EMAIL=sddadada
      - CLOUDFLARE_DNS_API_TOKEN=dadadadad
      
    networks:
      - ovencrypt
      
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - certs:/letsencrypt
    
    ports:
      - 80:80
      - 443:443
        
      
    deploy:
      mode: global  
      labels:
        - "traefik.enable=true"
        - "traefik.http.routers.traefikae.tls=true"
        - "traefik.http.routers.traefikae.tls.domains[0].main=micha.africa"
        - "traefik.http.routers.traefikae.tls.domains[0].sans=*.micha.africa"
        - "traefik.http.routers.traefikae.tls.certresolver=letsencrypt"
        
        - "traefik.http.routers.traefikae.rule=Host(`traefik.micha.africa`)" 
        - "traefik.http.services.traefikae.loadbalancer.server.port=8080"
        - "traefik.docker.network=ovencrypt"
        - "traefik.http.middlewares.traefikae-auth.basicauth.users=micha:$$2y$$05$$NrR4hl3V7uCFT8nOdc5ZC.1AHuTjx4ysafhpBe2s0xX12eCG81VUO"
        - "traefik.http.routers.traefikae.middlewares=traefikae-auth"
          

volumes:
  certs:
  agent-volume:
  portainer-data:
    
networks:
  ovencrypt:
    external: true
    attachable: true
    
  agent-network:
```


## Traefik + LetsEncrypt + Grafana + Node Exporter + Cadvisor (Monitoring Metrics)

```yml
version: "3.9"

services:
  cadvisor:
    image: docker
    deploy:
      mode: global
    volumes: 
        - /var/run/docker.sock:/var/run/docker.sock:ro
    entrypoint: ["/bin/sh","-c"]
    environment:
      - PARENT={{.Task.Name}}
      - CHILDNAME={{.Service.Name}}_sidecar.{{.Node.ID}}.{{.Task.ID}}
      - CADVISOR_VERSION=v0.37.5
   
    ports:
      - "8080:8080"
    networks:
      - ovencrypt
      
    command: 
    - |
      exec docker run -i --rm --network="container:$${PARENT}" \
            --volume=/:/rootfs:ro \
            --volume=/var/run:/var/run:ro  \
            --volume=/sys:/sys:ro  \
            --volume=/var/lib/docker/:/var/lib/docker:ro \
            --volume=/dev/disk/:/dev/disk:ro \
            --name $${CHILDNAME} \
            --privileged \
            --device=/dev/kmsg \
            gcr.io/cadvisor/cadvisor:$${CADVISOR_VERSION}
    
  node_exporter:
    image: quay.io/prometheus/node-exporter:v1.5.0
    command: "--path.rootfs=/host"
    pid: host
    restart: unless-stopped
    networks:
      - ovencrypt
    volumes:
      - /:/host:ro,rslave
      
  grafana:
    image: grafana/grafana-oss:latest
    
    networks:
      - ovencrypt
  
    volumes:
      - grafana-data:/var/lib/grafana
    
    ports:
       - 4000:3000
    deploy:
      replicas: 1
      labels:
        - "traefik.enable=true"
        - "traefik.http.routers.grafana.rule=Host(`grafana.micha.africa`)" 
        - "traefik.http.services.grafana.loadbalancer.server.port=3000"
        - "traefik.docker.network=ovencrypt"
        - "traefik.http.routers.grafana.tls=true"
        - "traefik.http.routers.grafana.tls.certresolver=letsencrypt"
    
  prometheus:
    image: prom/prometheus:latest
    networks:
      - ovencrypt
    
    volumes:
      - /etc/prometheus:/etc/prometheus
      - prometheus-data:/prometheus
    command: "--config.file=/etc/prometheus/prometheus.yml"
    ports:
      - 9090:9090
    deploy:
      placement:
        constraints: 
          - node.role == manager
      replicas: 1
      labels:
        - "traefik.enable=true"
        - "traefik.http.routers.prometheus.rule=Host(`prometheus.micha.africa`)" 
        - "traefik.http.services.prometheus.loadbalancer.server.port=9090"
        - "traefik.docker.network=ovencrypt"
        - "traefik.http.routers.prometheus.tls=true"
        - "traefik.http.routers.prometheus.tls.certresolver=letsencrypt"
    
        
  traefik:
    image: traefik:latest
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

      # Letsencrypt setup HTTP CHALLENGE
      - --certificatesresolvers.myresolver.acme.httpchallenge=true
      - --certificatesResolvers.myresolver.acme.httpChallenge.entryPoint=httpa
      - --certificatesresolvers.myresolver.acme.email=michameiu@gmail.com
      - --certificatesresolvers.myresolver.acme.storage=/letsencrypt/acme2.json

      # Letsencrypt setup DNS CHALLENGE
      - --certificatesresolvers.letsencrypt.acme.dnschallenge=true
      - --certificatesresolvers.letsencrypt.acme.dnschallenge.provider=cloudflare
      - --certificatesresolvers.le.acme.dnschallenge.resolvers=1.1.1.1:53,8.8.8.8:53
      - --certificatesresolvers.letsencrypt.acme.email=michameiu@gmail.com
      - --certificatesresolvers.letsencrypt.acme.storage=/letsencrypt/acme.json
      
      # Set up an insecure listener that redirects all traffic to TLS
      - --entrypoints.websecure.address=:443      
      - --entrypoints.httpa.http.redirections.entrypoint.to=websecure
      - --entrypoints.httpa.http.redirections.entrypoint.scheme=https
      # Set up the TLS configuration for our websecure listener
      - --entrypoints.websecure.http.tls=true
      - --entrypoints.websecure.http.tls.certResolver=letsencrypt
      
     
      
    
    environment:
      - CLOUDFLARE_EMAIL=alkdjladljjns
      - CLOUDFLARE_DNS_API_TOKEN=daduyauydga
      
    networks:
      - ovencrypt
      
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - certs:/letsencrypt
    
    ports:
      - 80:80
      - 443:443
        
      
    deploy:
      mode: global  
      labels:
        - "traefik.enable=true"
        - "traefik.http.routers.traefikae.tls=true"
        - "traefik.http.routers.traefikae.tls.domains[0].main=micha.africa"
        - "traefik.http.routers.traefikae.tls.domains[0].sans=*.micha.africa"
        - "traefik.http.routers.traefikae.tls.certresolver=letsencrypt"
        
        - "traefik.http.routers.traefikae.rule=Host(`traefik.micha.africa`)" 
        - "traefik.http.services.traefikae.loadbalancer.server.port=8080"
        - "traefik.docker.network=ovencrypt"
        - "traefik.http.middlewares.traefikae-auth.basicauth.users=micha:$$2y$$05$$NrR4hl3V7uCFT8nOdc5ZC.1AHuTjx4ysafhpBe2s0xX12eCG81VUO"
        - "traefik.http.routers.traefikae.middlewares=traefikae-auth"
          

volumes:
  grafana-data:
  prometheus-data:
  certs:
  agent-volume:
  portainer-data:
    
networks:
  host:
    external: true
  ovencrypt:
    external: true
    attachable: true
    
  agent-network:
```

### Prometheus config
```yml title="/etc/prometheus/prometheus.yml"
global:
  scrape_interval:     15s # By default, scrape targets every 15 seconds.

  # Attach these labels to any time series or alerts when communicating with
  # external systems (federation, remote storage, Alertmanager).
  # external_labels:
  #  monitor: 'codelab-monitor'

# A scrape configuration containing exactly one endpoint to scrape:
# Here it's Prometheus itself.
scrape_configs:
  # The job name is added as a label `job=<job_name>` to any timeseries scraped from this config.
  - job_name: 'prometheus'
    # Override the global default and scrape targets from this job every 5 seconds.
    scrape_interval: 5s
    static_configs:
      - targets: ['localhost:9090']

  - job_name: 'node_exporter'
    static_configs:
      - targets: ['node_exporter:9100']

  - job_name: 'cadvisor'
    static_configs:
      - targets: ['cadvisor:8080']
```



## Traefik Monitoring

```yml
version: "3.9"

services:
  traefika:
    image: traefik:latest
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
      ## MEtrics
      - --metrics.prometheus=true
      - --entryPoints.metrics.address=:8082
      - --metrics.prometheus.buckets=0.1,0.3,1.2,5.0
      - --metrics.prometheus.addEntryPointsLabels=true
      - --metrics.prometheus.addrouterslabels=true
      - --metrics.prometheus.addServicesLabels=true
      - --metrics.prometheus.entryPoint=metrics
      ...
    deploy:
      mode: global
      placement:
        constraints: 
          - node.role == manager      
      labels:
        - "traefik.enable=true"
        - "traefik.http.routers.traefikae.rule=Host(`traefik.example.com`)" 
        - "traefik.http.services.traefikae.loadbalancer.server.port=8080"
        - "traefik.http.routers.traefikae.service=traefikae"
        - "traefik.docker.network=ovencrypt"
        - "traefik.http.middlewares.traefikae-auth.basicauth.users=micha:$$2y$$05$$NrR4hl3V7uCFT8nOdc5ZC.1AHuTjx4ysafhpBe2s0xX12eCG81VUO"
        - "traefik.http.routers.traefikae.middlewares=traefikae-auth"
        
        - "traefik.http.routers.metrics.rule=Host(`metrics.example.com`)" 
        - "traefik.http.services.metrics.loadbalancer.server.port=8082"
        - "traefik.http.routers.metrics.service=metrics"
```