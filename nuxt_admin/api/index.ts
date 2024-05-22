

/**
 * @author wcz
 */
export function logout() {
  return request.post(`/wccloud-admin-server/auth/logout`);
}
