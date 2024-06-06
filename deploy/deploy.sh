#!/bin/bash
echo '开始构建'
cd deploy || exit
pwd
npm config set registry https://registry.npmmirror.com
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

#rust
if [ "$(echo $works|grep '#wccloud-web-rust#')" != "" ]; then
  echo '##########开始构建wccloud-web-rust##########'
  cd ..
  cd wccloud-web-rust || exit
  #/root/.cargo/bin/cargo  build --release
  if [ "$(docker ps|grep wccloud-web-rust)" != "" ]; then
    echo  'stop rm rmi '
   docker stop wccloud-web-rust && docker rm wccloud-web-rust && docker rmi wccloud-web-rust
  fi
  cd ..
  cd deploy || exit
  docker compose -f wccloud-web-rust.yml up -d
fi