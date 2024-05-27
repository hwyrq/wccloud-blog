// Parameter interface


/**
 * @author wcz
 */
export function login(params: any) {
    return request.post(`/wccloud-admin-server/auth/login`, params);
}

export function resetPwd(params: any) {
    return request.post(`/wccloud-admin-server/auth/reset-pwd`, params);
}
