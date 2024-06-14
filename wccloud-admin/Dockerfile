FROM openjdk:21-slim
ENV ARGS=""
#ENV JAVA_OPTS="-Xms128m -Xmx128m -javaagent:/home/skywalking-agent/skywalking-agent.jar -DSW_AGENT_NAME=wccloud-admin -DSW_AGENT_COLLECTOR_BACKEND_SERVICES=192.168.1.22:11800 "
ENV JAVA_OPTS="-Xms128m -Xmx128m  -Dspring.cloud.nacos.discovery.namespace=wccloud-pro "
EXPOSE 8084
ADD ./build/libs/wccloud-admin-0.0.1-SNAPSHOT.jar wccloud-admin-0.0.1-SNAPSHOT.jar
CMD java ${JAVA_OPTS} -jar wccloud-admin-0.0.1-SNAPSHOT.jar $ARGS