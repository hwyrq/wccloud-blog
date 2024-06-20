FROM registry.cn-shanghai.aliyuncs.com/hwyrq/openjdk
ENV ARGS=""
ENV JAVA_OPTS="-Xms64m -Xmx64m -Dspring.cloud.nacos.discovery.namespace=wccloud-pro -javaagent:/home/skywalking-agent/skywalking-agent.jar -DSW_AGENT_NAME=wccloud-auth -DSW_AGENT_COLLECTOR_BACKEND_SERVICES=192.168.1.22:11800 "

EXPOSE 8082
ADD ./build/libs/wccloud-auth-0.0.1-SNAPSHOT.jar wccloud-auth-0.0.1-SNAPSHOT.jar
CMD java ${JAVA_OPTS} -jar wccloud-auth-0.0.1-SNAPSHOT.jar $ARGS