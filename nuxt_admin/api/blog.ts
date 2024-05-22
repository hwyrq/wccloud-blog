// Response interface

/**
 * @author wcz
 */
export function page(params:any){
    return request.get(`/wccloud-web-rust/blog/page`,{params:params});
}
export function one(params:any){
    return request.get(`/wccloud-web-rust/blog/one`,{params:params});
}
export function save(params: any) {
    return request.post(`/wccloud-web-rust/blog/save`, params);
}

export function del(params: any) {
    return request.delete(`/wccloud-web-rust/blog/delete?blogId=`+params.blogId);
}
export function listLabelName(){
    return request.get(`/wccloud-web-rust/label/list/name`);
}
export function listTypeName(){
    return request.get(`/wccloud-web-rust/type/list/name`);
}