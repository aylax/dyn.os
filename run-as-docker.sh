#!/bin/bash
docker stop $(docker ps -aq) && docker rm $(docker ps -aq)
docker rmi $(docker images | grep "none" | awk '{print $3}')
docker build -t aylax:dyn-os-basic -f docker.d/basic .
docker build -t aylax:dyn-os-after -f docker.d/after .
docker build -t aylax:dyn-os-final -f docker.d/final .
docker run -d -p 127.0.0.1:6000:6000/udp --name dyn-os aylax:dyn-os-final

# run underline command
# echo "send udp data" > /dev/udp/127.0.0.1/6000

