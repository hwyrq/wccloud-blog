#!/bin/bash
# author wcz
#当前jenkins部署在docker内，构建直接执行当前deploy.sh文件即可，java需要提前安装好gradle,以及nodejs
#当前只打包镜像并推送到阿里云
#宿主机应当先设置时区 echo 'Asia/Shanghai' > /etc/timezone
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

#手动修改这里，临时重新部署某一个
works='#wccloud-admin# #wccloud-auth# #wccloud-gateway# #wccloud-web-rust# #nuxt_admin# #nuxt_web# '
works='#wccloud-web-rust#   '
echo "works: "
echo $works
#java
if [ "$(echo $works|grep '#wccloud-admin#')" != "" ]; then
  echo '##########开始构建wccloud-admin##########'
  package
  if [ "$(docker ps|grep wccloud-admin)" != "" ]; then
    echo  'stop rm rmi '
#   docker stop wccloud-admin && docker rm wccloud-admin && docker rmi registry.cn-shanghai.aliyuncs.com/wccloud/wccloud-admin
  fi
  docker compose -f wccloud-admin.yml build
  docker push registry.cn-shanghai.aliyuncs.com/wccloud/wccloud-admin
fi

if [ "$(echo $works|grep '#wccloud-auth#')" != "" ]; then
  echo '##########开始构建wccloud-auth##########'
  package
  if [ "$(docker ps|grep wccloud-auth)" != "" ]; then
    echo  'stop rm rmi '
#   docker stop wccloud-auth && docker rm wccloud-auth && docker rmi registry.cn-shanghai.aliyuncs.com/wccloud/wccloud-auth
  fi
  docker compose -f wccloud-auth.yml build
  docker push registry.cn-shanghai.aliyuncs.com/wccloud/wccloud-auth
fi

if [ "$(echo $works|grep '#wccloud-gateway#')" != "" ]; then
  echo '##########开始构建wccloud-gateway##########'
  package
  if [ "$(docker ps|grep wccloud-gateway)" != "" ]; then
    echo  'stop rm rmi '
#    docker rmi registry.cn-shanghai.aliyuncs.com/wccloud/wccloud-gateway
  fi
  docker compose -f wccloud-gateway.yml build
  docker push registry.cn-shanghai.aliyuncs.com/wccloud/wccloud-gateway
fi

#rust
if [ "$(echo $works|grep '#wccloud-web-rust#')" != "" ]; then
  echo '##########开始构建wccloud-web-rust##########'
  cd ..
  cd wccloud-web-rust || exit
  #jenkins内不能直接用cargo命令，所以这里就这样吧
  /root/.cargo/bin/cargo  build --release
  if [ "$(docker ps|grep wccloud-web-rust)" != "" ]; then
    echo  'stop rm rmi '
#   docker stop wccloud-web-rust && docker rm wccloud-web-rust && docker rmi registry.cn-shanghai.aliyuncs.com/wccloud/wccloud-web-rust
  fi
  cd ..
  cd deploy || exit
  docker compose -f wccloud-web-rust.yml build
  docker push registry.cn-shanghai.aliyuncs.com/wccloud/wccloud-web-rust
fi

#node
#配置npm加速代理
npm config set registry https://registry.npmmirror.com
npm -version
node --version

if [ "$(echo $works|grep '#nuxt_admin#')" != '' ]; then
  echo '##########开始构建nuxt_admin##########'
  cd ..
  cd nuxt_admin || exit
  pwd
  echo 'npm install...'
  npm install
  echo 'npm run build...'
  npm run build
#  if [ "$(docker ps|grep nuxt_admin)" != "" ]; then
#   docker stop nuxt_admin && docker rm nuxt_admin && docker rmi registry.cn-shanghai.aliyuncs.com/wccloud/nuxt_admin
#  fi
  cd ..
  cd deploy || exit
  echo 'docker compose -f nuxt_admin.yml build'
  docker compose -f nuxt_admin.yml build
  docker push registry.cn-shanghai.aliyuncs.com/wccloud/nuxt_admin
fi
if [ "$(echo $works|grep '#nuxt_web#')" != '' ]; then
  echo '##########开始构建nuxt_web##########'
  cd ..
  cd nuxt_web || exit
  npm install
  npm run build
#  if [ "$(docker ps|grep nuxt_web)" != "" ]; then
#   docker stop nuxt_web && docker rm nuxt_web && docker rmi registry.cn-shanghai.aliyuncs.com/wccloud/nuxt_web
#  fi
  cd ..
  cd deploy || exit
  docker compose -f nuxt_web.yml build
  docker push registry.cn-shanghai.aliyuncs.com/wccloud/nuxt_web
fi

