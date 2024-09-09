module.exports = {
    apps: [
        {
            name: 'nuxi_web',
            port: '8000',
            exec_mode: 'cluster',
            instances: '1',
            script: './.output/server/index.mjs',
            // max_memory_restart: '100M',
            env:{
                BASE_URL:'http://www.wccloud.top/gateway',
                BASE_URL_IN:'http://10.96.2.0:8081'
            }
        }
    ]
}
