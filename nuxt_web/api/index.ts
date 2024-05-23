/**
 * @author wcz
 */
export function page(params:any){
    return request.get(`/wccloud-web-rust/anonymous/blog/page`,{params:params});
}

export function one(params:any){
    return request.get(`/wccloud-web-rust/anonymous/blog/one`,{params:params});
}
export function level(params:any){
    return request.get(`/wccloud-web-rust/anonymous/blog/level`,{params:params});
}
export function label(params:any){
    return request.get(`/wccloud-web-rust/anonymous/blog/label`,{params:params});
}