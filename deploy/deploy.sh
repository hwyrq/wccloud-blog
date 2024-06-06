#!/bin/bash
# author wcz
#当前jenkins部署在docker内，构建直接执行当前deploy.sh文件即可，java需要提前安装好gradle,以及nodejs
echo '开始构建'
cd deploy || exit
pwd

gradle_package=0
package(){
  if [ $gradle_package = 0 ]; then
    cd ..
    gradle_package=1
    JAVA_HOME=/var/jenkins_home/graalvm-community-openjdk-21
    /var/jenkins_home/tools/hudson.plugins.gradle.GradleInstallation/gradle8.6/bin/gradle bootJar
    cd deploy || exit
    fi
}
#获取git提交的日志，提交了那个服务，就部署哪个服务，未变更的则不会部署
works=$(git log -1 --stat |awk -F ' ' '{print $1}' |awk -F '/' '{print $1}' | awk '$0="#"$0"#"')
echo "works: "
echo $works
#java
#if [ "$(echo $works|grep '#wccloud-admin#')" != "" ]; then
#  echo '##########开始构建wccloud-admin##########'
#  package
#  if [ "$(docker ps|grep wccloud-admin)" != "" ]; then
#    echo  'stop rm rmi '
#   docker stop wccloud-admin && docker rm wccloud-admin && docker rmi wccloud-admin
#  fi
#  docker compose -f wccloud-admin.yml up -d
#fi
#
#if [ "$(echo $works|grep '#wccloud-auth#')" != "" ]; then
#  echo '##########开始构建wccloud-auth##########'
#  package
#  if [ "$(docker ps|grep wccloud-auth)" != "" ]; then
#    echo  'stop rm rmi '
#   docker stop wccloud-auth && docker rm wccloud-auth && docker rmi wccloud-auth
#  fi
#  docker compose -f wccloud-auth.yml up -d
#fi
#
#if [ "$(echo $works|grep '#wccloud-gateway#')" != "" ]; then
#  echo '##########开始构建wccloud-gateway##########'
#  package
#  if [ "$(docker ps|grep wccloud-gateway)" != "" ]; then
#    echo  'stop rm rmi '
#   docker stop wccloud-gateway && docker rm wccloud-gateway && docker rmi wccloud-gateway
#  fi
#  docker compose -f wccloud-gateway.yml up -d
#fi
#
##rust
#if [ "$(echo $works|grep '#wccloud-web-rust#')" != "" ]; then
#  echo '##########开始构建wccloud-web-rust##########'
#  cd ..
#  cd wccloud-web-rust || exit
#  #/root/.cargo/bin/cargo  build --release
#  if [ "$(docker ps|grep wccloud-web-rust)" != "" ]; then
#    echo  'stop rm rmi '
#   docker stop wccloud-web-rust && docker rm wccloud-web-rust && docker rmi wccloud-web-rust
#  fi
#  cd ..
#  cd deploy || exit
#  docker compose -f wccloud-web-rust.yml up -d
#  #将docker内构建的依赖以及缓存复制到 jenkins内的项目目录，提升下次编译速度
#  docker cp  wccloud-web-rust:/usr/local/cargo /var/jenkins_home/workspace/wccloud/wccloud-web-rust/
#  docker cp  wccloud-web-rust:/wccloud-web-rust/target /var/jenkins_home/workspace/wccloud/wccloud-web-rust/
#fi

#node
#配置npm加速代理
npm config set registry https://registry.npmmirror.com

#if [ "$(echo $works|grep 'nuxi_admin')" != '' ]; then
  echo '##########开始构建nuxi_admin##########'
  cd nuxi_admin || exit
  npm install
  npm run build
  docker stop nuxi_admin && docker rm nuxi_admin && docker rmi nuxi_admin
  cd ..
  cd deploy || exit
  docker compose -f nuxi_admin.yml up -d
#fi
#if [ "$(echo $works|grep 'nuxi_web')" != '' ]; then
#  echo '##########开始构建nuxi_web##########'
#  cd nuxi_web || exit
#  npm install
#  npm run build
#  docker stop nuxi_web && docker rm nuxi_web && docker rmi nuxi_web
#  cd ..
#  cd deploy || exit
#  docker compose -f nuxi_web.yml up -d
#fi