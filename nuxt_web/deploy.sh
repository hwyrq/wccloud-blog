#!/bin/bash
echo '开始构建'
pwd

node --version

npm -version

npm config set registry https://registry.npmmirror.com

cd nuxi_web || exit

npm install

npm run build

docker stop nuxi_web && docker rm nuxi_web && docker rmi nuxi_web

docker compose -f nuxi_web.yml up -d
