# Keptn

## What is Keptn

Based on kd3 lightweight kubernetes

## Installation Requirements

### Install docker

1. Update the apt package index and install packages to allow apt to use a repository over HTTPS:

    ```
    sudo apt-get update
    sudo apt-get install \
        ca-certificates \
        curl \
        gnupg \
        lsb-release
    ```

1. Add Docker’s official GPG key:

    ```
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
    ```

1. Use the following command to set up the stable repository.

    ```
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] <https://download.docker.com/linux/ubuntu> \
    $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
    ```

1. Install docker engine

    ```
    sudo apt-get update
    sudo apt-get install docker-ce docker-ce-cli containerd.io
    ```

## Errors

```
sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 40976EAF437D05B5
```

## Install kubectl

1. Download the latest release

    ```
    curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
    ```

1. Validate the binary (optional)

    ```
    curl -LO "https://dl.k8s.io/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl.sha256"
    ```

    ```
    echo "$(cat kubectl.sha256)  kubectl" | sha256sum --check
    ```

1. Install kubectl

    ```
    sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl
    ```

## Install Helm

Kubernetes Package manager

1. Download get helm script

    ```
    curl -fsSL -o get_helm.sh https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3
    ```

1. Set the permissions

    ```
    chmod 700 get_helm.sh
    ```

1. Run Get Helm

    ```
    ./get_helm.sh
    ```

### Install k3d

k3d Is a lightweight kubernetes
    ```
    curl -s https://raw.githubusercontent.com/k3d-io/k3d/main/install.sh | bash
    ```

## Installing Keptn

```
curl -s https://raw.githubusercontent.com/rancher/k3d/main/install.sh | TAG=v5.3.0 bash
k3d cluster create mykeptn -p "8082:80@loadbalancer" --k3s-arg "--no-deploy=traefik@server:*"
```
