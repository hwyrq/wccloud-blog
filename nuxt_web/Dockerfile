FROM registry.cn-shanghai.aliyuncs.com/hwyrq/node
ADD .output /.output
ADD ecosystem.config.js /ecosystem.config.js
EXPOSE 8000
RUN npm config set registry https://registry.npmmirror.com
RUN npm install pm2 -g
CMD pm2-runtime ecosystem.config.js
