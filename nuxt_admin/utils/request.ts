import axios from 'axios'
import type {Action} from "element-plus";
import jsonBig from "json-bigint";

/**
 * @author wcz
 */
const requester = axios.create({
    baseURL: process.env.BASE_URL || useRuntimeConfig().public.baseUrl,
    timeout: 15000,
    transformResponse: data => {
        try {
            return jsonBig({"storeAsString": true}).parse(data);
        } catch (err) {
            console.log(err);
            return JSON.parse(data)
        }
    }
})
requester.interceptors.request.use(
    config => {
        console.log("config");
        config.headers['Token'] = localStorage.getItem("accessToken")
        return config
    },
    err => {
        return Promise.reject(err)
    }
)
requester.interceptors.response.use(
    response => {
        if (response.data.code == -2) {
            //重新登录
            ElMessageBox.alert(response.data.msg, '提示', {
                confirmButtonText: 'OK',
                callback: (action: Action) => {
                    localStorage.setItem("accessToken", "1");
                    useRouter().push("/login");
                },
            });
            return Promise.reject('error');
        }
        return response.data;
    },
    error => {
        return Promise.reject(error.response)
    }
)
export default requester
