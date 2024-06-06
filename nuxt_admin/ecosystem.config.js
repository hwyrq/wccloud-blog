module.exports = {
    apps: [
        {
            name: 'nuxi_admin',
            port: '8001',
            exec_mode: 'cluster',
            instances: '6',
            script: './.output/server/index.mjs'
        }
    ]
}
