FROM registry.cn-shanghai.aliyuncs.com/hwyrq/openjdk
ENV ARGS=""
ENV JAVA_OPTS="-Xms64m -Xmx64m -Dspring.cloud.nacos.discovery.namespace=wccloud-pro -javaagent:/home/skywalking-agent/skywalking-agent.jar -DSW_AGENT_NAME=wccloud-gateway -DSW_AGENT_COLLECTOR_BACKEND_SERVICES=192.168.1.22:11800 "

EXPOSE 8081
ADD ./build/libs/wccloud-gateway-0.0.1-SNAPSHOT.jar wccloud-gateway-0.0.1-SNAPSHOT.jar
CMD java ${JAVA_OPTS} -jar wccloud-gateway-0.0.1-SNAPSHOT.jar $ARGS