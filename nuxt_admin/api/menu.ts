// Response interface
/**
 * @author wcz
 */
/**
 * 查询所有开启菜单
 * @returns
 */
export function listStatus0(){
    return request.get(`/wccloud-admin-server/menu/list-status0`);
}
/**
 * 查询所有菜单
 * @returns
 */
export function list() {
    return request.get(`/wccloud-admin-server/menu/list`);
}
/**
 * 新增菜单
 */
export function save(params: any) {
    return request.post(`/wccloud-admin-server/menu/save`, params);
}

/**
 * 更新菜单
 */
export function update(params: any) {
    return request.put(`/wccloud-admin-server/menu/update`, params);
}

/**
 * 删除菜单
 * @param {string} menuId
 * @returns
 */
export function del(menuId: number) {
    return request.delete(`/wccloud-admin-server/menu/delete?menuId=${menuId}`);
}
