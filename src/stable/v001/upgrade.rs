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
        let admin_users = if old_permitted.is_empty() {
            Vec::new()
        } else {
            state
                .canister_kit
                .permissions
                .user_permissions
                .iter()
                .filter_map(|(user_id, permissions)| (permissions == &old_permitted).then_some(*user_id))
                .collect::<Vec<_>>()
        };
        let admin_roles = if old_permitted.is_empty() {
            Vec::new()
        } else {
            state
                .canister_kit
                .permissions
                .role_permissions
                .iter()
                .filter_map(|(role, permissions)| (permissions == &old_permitted).then_some(role.clone()))
                .collect::<Vec<_>>()
        };

        let new_permissions = super::permission::get_all_permissions(|name| state.parse_permission(name));
        let new_permitted = ic_canister_kit::functions::permission::basic::permitted_permissions(&new_permissions);
        state.permission_reset(new_permissions);
        for user_id in admin_users {
            state
                .canister_kit
                .permissions
                .user_permissions
                .insert(user_id, new_permitted.clone());
        }
        for role in admin_roles {
            state
                .canister_kit
                .permissions
                .role_permissions
                .insert(role, new_permitted.clone());
        }

        Box::new(state)
    }
}
