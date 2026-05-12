use super::super::v000::types::{CanisterKit as LastCanisterKit, InnerState as LastState};

use super::types::*;

impl From<Box<LastState>> for Box<InnerState> {
    fn from(value: Box<LastState>) -> Self {
        let mut state = InnerState::default(); // ? 初始化

        // ! 每次升级新版本，务必比较每一个数据的升级方式
        // ! 如果不修改数据结构，可以直接赋值升级
        // ! 如果修改数据结构，必须代码处理数据升级

        // 1. 继承之前的数据
        let LastCanisterKit {
            pause,
            permissions,
            records,
            schedule,
        } = value.canister_kit;
        state.canister_kit.pause = pause;
        state.canister_kit.permissions = permissions;
        state.canister_kit.records = records;
        state.canister_kit.schedule = schedule;

        // 2. 刷新到最新权限集合，并只给旧版明确的管理员补齐新权限
        let old_permissions = state.canister_kit.permissions.permissions.clone();
        let old_permitted = ic_canister_kit::functions::permission::basic::permitted_permissions(&old_permissions);
        let super_users = super_keys(&state.canister_kit.permissions.user_permissions, &old_permitted);
        let super_roles = super_keys(&state.canister_kit.permissions.role_permissions, &old_permitted);

        let new_permissions = super::permission::get_all_permissions(|name| state.parse_permission(name));
        let new_permitted = ic_canister_kit::functions::permission::basic::permitted_permissions(&new_permissions);
        let mut updated = Vec::with_capacity(super_users.len() + super_roles.len());
        let users_updated = super_users
            .into_iter()
            .map(|user_id| PermissionUpdatedArg::UpdateUserPermission(user_id, Some(new_permitted.clone())));
        let roles_updated = super_roles
            .into_iter()
            .map(|role| PermissionUpdatedArg::UpdateRolePermission(role, Some(new_permitted.clone())));
        updated.extend(users_updated);
        updated.extend(roles_updated);

        // 刷新权限
        state.permission_reset(new_permissions);
        if !updated.is_empty() {
            assert!(state.permission_update(updated).is_ok()); // 插入权限
        }

        Box::new(state)
    }
}

fn super_keys<K>(
    permission_map: &std::collections::HashMap<K, std::collections::HashSet<Permission>>,
    full_permissions: &std::collections::HashSet<Permission>,
) -> Vec<K>
where
    K: Clone,
{
    if full_permissions.is_empty() {
        return Vec::new();
    }

    permission_map
        .iter()
        .filter_map(|(key, permissions)| (permissions == full_permissions).then_some(key.clone()))
        .collect()
}
