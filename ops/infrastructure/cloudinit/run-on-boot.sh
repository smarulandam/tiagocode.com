#!/bin/bash

echo "${PROJECT_NAME} starting ..."

set -euo pipefail

sudo dnf update
sudo dnf install wget ruby -y

# Install Basic utilities
sudo dnf install telnet htop  -y

# Install Code Deploy agent
cd /home/ec2-user
wget https://aws-codedeploy-${AWS_REGION}.s3.${AWS_REGION}.amazonaws.com/latest/install
chmod +x ./install
sudo ./install auto
sudo systemctl enable codedeploy-agent
sudo systemctl start codedeploy-agent

# Install Docker & Docker Compose plugin
sudo dnf install docker -y
sudo service docker start
sudo usermod -aG docker ec2-user
newgrp docker
sudo systemctl enable docker.service
sudo systemctl enable containerd.service
sudo curl -L https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m) -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
sudo docker-compose --version

# Install Postgres
 sudo dnf install postgresql17 -y

 # Add SWAP
sudo dd if=/dev/zero of=/swapfile bs=128M count=32
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
echo '/swapfile swap swap defaults 0 0' | sudo tee -a /etc/fstab
sudo swapon -s
free -h
