module.exports = {
    apps: [
        {
            name: 'nuxi_web',
            port: '3001',
            exec_mode: 'cluster',
            instances: '6',
            script: './.output/server/index.mjs'
        }
    ]
}
