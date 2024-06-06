module.exports = {
    apps: [
        {
            name: 'nuxi_web',
            port: '8000',
            exec_mode: 'cluster',
            instances: '6',
            script: './.output/server/index.mjs'
        }
    ]
}
