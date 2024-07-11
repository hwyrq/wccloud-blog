FROM registry.cn-shanghai.aliyuncs.com/hwyrq/openjdk
ENV ARGS=""
ENV JAVA_OPTS="-Xms256m -Xmx256m -Dspring.cloud.nacos.discovery.namespace=wccloud-pro -javaagent:/home/skywalking-agent/skywalking-agent.jar -DSW_AGENT_NAME=wccloud-admin -DSW_AGENT_COLLECTOR_BACKEND_SERVICES=192.168.1.22:11800 "
EXPOSE 8084
ADD ./build/libs/wccloud-admin-0.0.1-SNAPSHOT.jar wccloud-admin-0.0.1-SNAPSHOT.jar
CMD java ${JAVA_OPTS} -jar wccloud-admin-0.0.1-SNAPSHOT.jar $ARGS