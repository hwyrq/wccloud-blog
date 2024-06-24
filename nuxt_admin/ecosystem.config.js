module.exports = {
    apps: [
        {
            name: 'nuxi_admin',
            port: '8001',
            exec_mode: 'cluster',
            instances: '1',
            script: './.output/server/index.mjs',
            max_memory_restart: '100M',
            env:{
                BASE_URL:'http://www.wccloud.top/gateway'
            }
        }
    ]
}
