import axios from 'axios';
import jsonBig from "json-bigint";

/**
 * author wcz
 */
const requester_in = axios.create({
    baseURL: process.env.BASE_URL_IN as string ?? useRuntimeConfig().public.baseUrlIn,
    timeout: 15000,
    transformResponse: data => {
        try {
            return jsonBig({"storeAsString": true}).parse(data);
        } catch (err) {
            return JSON.parse(data)
        }
    }
})
requester_in.interceptors.request.use(
    config => {
        return config
    },
    err => {
        return Promise.reject(err)
    }
)
requester_in.interceptors.response.use(
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
export default requester_in
