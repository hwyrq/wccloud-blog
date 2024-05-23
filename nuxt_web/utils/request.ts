
import axios from 'axios'
import jsonBig from "json-bigint";
const requester = axios.create({
    baseURL: 'http://127.0.0.1:8081',
    timeout: 15000,
    transformResponse: data => {
        try{
            return jsonBig({"storeAsString": true}).parse(data);
        }catch (err) {
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
