// Parameter interface


/**
 * @author wcz
 */
export function login(params: any) {
    return request.post(`/wccloud-admin-server/auth/login`, params);
}

