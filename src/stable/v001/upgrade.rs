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

#[cfg(test)]
mod tests {
    use super::super::super::v000::types as v000_types;
    use super::*;

    fn v000_permissions() -> std::collections::HashSet<Permission> {
        let state = LastState::default();
        let permissions = v000_types::ACTIONS
            .iter()
            .filter_map(|name| state.parse_permission(name).ok())
            .collect::<std::collections::HashSet<_>>();
        assert_eq!(permissions.len(), v000_types::ACTIONS.len());
        permissions
    }

    fn migrate(last_state: LastState) -> Box<InnerState> {
        Box::new(last_state).into()
    }

    #[test]
    fn should_refresh_permission_set_to_latest() {
        let mut last_state = LastState::default();
        last_state.permission_reset(v000_permissions());

        let state = migrate(last_state);

        assert!(
            state
                .canister_kit
                .permissions
                .permissions
                .contains(&Permission::by_permit(ACTION_BUSINESS_UPLOAD))
        );
    }

    #[test]
    fn should_grant_new_permissions_to_super_user() {
        let user = UserId::from_slice(&[1]);
        let mut last_state = LastState::default();
        let old_permissions = v000_permissions();
        let old_permitted = ic_canister_kit::functions::permission::basic::permitted_permissions(&old_permissions);
        last_state.permission_reset(old_permissions);
        assert!(
            last_state
                .permission_update(vec![PermissionUpdatedArg::UpdateUserPermission(
                    user,
                    Some(old_permitted)
                )])
                .is_ok()
        );

        let state = migrate(last_state);
        let assigned = state.canister_kit.permissions.user_permissions.get(&user);

        assert!(
            assigned.is_some_and(|permissions| permissions.contains(&Permission::by_permit(ACTION_BUSINESS_UPLOAD)))
        );
    }

    #[test]
    fn should_not_grant_new_permissions_to_partial_user() {
        let user = UserId::from_slice(&[2]);
        let mut last_state = LastState::default();
        let old_permissions = v000_permissions();
        let partial_permissions = [Permission::by_permit(v000_types::ACTION_PAUSE_REPLACE)]
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        last_state.permission_reset(old_permissions);
        assert!(
            last_state
                .permission_update(vec![PermissionUpdatedArg::UpdateUserPermission(
                    user,
                    Some(partial_permissions)
                )])
                .is_ok()
        );

        let state = migrate(last_state);
        let assigned = state.canister_kit.permissions.user_permissions.get(&user);

        assert!(
            !assigned.is_some_and(|permissions| permissions.contains(&Permission::by_permit(ACTION_BUSINESS_UPLOAD)))
        );
    }

    #[test]
    fn should_grant_new_permissions_to_super_role() {
        let role = "Super".to_string();
        let mut last_state = LastState::default();
        let old_permissions = v000_permissions();
        let old_permitted = ic_canister_kit::functions::permission::basic::permitted_permissions(&old_permissions);
        last_state.permission_reset(old_permissions);
        assert!(
            last_state
                .permission_update(vec![PermissionUpdatedArg::UpdateRolePermission(
                    role.clone(),
                    Some(old_permitted)
                )])
                .is_ok()
        );

        let state = migrate(last_state);
        let assigned = state.canister_kit.permissions.role_permissions.get(&role);

        assert!(
            assigned.is_some_and(|permissions| permissions.contains(&Permission::by_permit(ACTION_BUSINESS_UPLOAD)))
        );
    }

    #[test]
    fn should_not_grant_new_permissions_to_partial_role() {
        let role = "Operator".to_string();
        let mut last_state = LastState::default();
        let old_permissions = v000_permissions();
        let partial_permissions = [Permission::by_permit(v000_types::ACTION_PAUSE_REPLACE)]
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        last_state.permission_reset(old_permissions);
        assert!(
            last_state
                .permission_update(vec![PermissionUpdatedArg::UpdateRolePermission(
                    role.clone(),
                    Some(partial_permissions)
                )])
                .is_ok()
        );

        let state = migrate(last_state);
        let assigned = state.canister_kit.permissions.role_permissions.get(&role);

        assert!(
            !assigned.is_some_and(|permissions| permissions.contains(&Permission::by_permit(ACTION_BUSINESS_UPLOAD)))
        );
    }
}
