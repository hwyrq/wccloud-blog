import axios from 'axios'
import jsonBig from "json-bigint";

/**
 * author wcz
 */
const requester = axios.create({
    //有一说一，这里真的坑，服务端请求用process.env.BASE_URL，客户端请求用useRuntimeConfig,反过来或者只用一个是不行的
    //也可能自己不是专业前端吧，落伍了
    baseURL: process.env.BASE_URL || useRuntimeConfig().public.baseUrl,
    timeout: 15000,
    transformResponse: data => {
        try {
            return jsonBig({"storeAsString": true}).parse(data);
        } catch (err) {
            return JSON.parse(data)
        }
    }
})
requester.interceptors.request.use(
    config => {
        return config
    },
    err => {
        return Promise.reject(err)
    }
)
requester.interceptors.response.use(
    response => {
        if (response.data.code !== 0) {
            return Promise.reject(response.data)
        } else {
            return response.data
        }
    },
    error => {
        return Promise.reject(error.response)
    }
)
export default requester
