module.exports = {
    apps: [
        {
            name: 'nuxi_web',
            port: '8000',
            exec_mode: 'cluster',
            instances: '2',
            script: './.output/server/index.mjs',
            max_memory_restart: '50M',
        }
    ]
}
