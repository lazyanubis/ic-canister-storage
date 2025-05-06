//! https://github.com/dfinity/pocketic
use candid::{Principal, encode_one};
use pocket_ic::PocketIc;

// 2T cycles
const INIT_CYCLES: u128 = 2_000_000_000_000;

const WASM_MODULE: &[u8] = include_bytes!("../sources/source_opt.wasm");

#[ignore]
#[test]
#[rustfmt::skip]
fn test_canister() {
    let pic = PocketIc::new();

    let default_identity = Principal::from_text("2ibo7-dia").unwrap();
    let alice_identity = Principal::from_text("uuc56-gyb").unwrap();
    let bob_identity = Principal::from_text("hqgi5-iic").unwrap(); // cspell: disable-line
    let carol_identity = Principal::from_text("jmf34-nyd").unwrap();
    let anonymous_identity = Principal::from_text("2vxsx-fae").unwrap();

    let template = Principal::from_text("lxzze-o7777-77777-aaaaa-cai").unwrap();

    pic.create_canister_with_id(Some(default_identity), None, template).unwrap();
    pic.add_cycles(template, INIT_CYCLES);

    pic.install_canister(template, WASM_MODULE.to_vec(), encode_one(None::<()>).unwrap(), Some(default_identity));

    use interface::*;
    use Permission::*;

    let pocketed_template = PocketedCanisterId::new(template, &pic);
    #[allow(unused)] let default = pocketed_template.sender(default_identity);
    #[allow(unused)] let alice = pocketed_template.sender(alice_identity);
    #[allow(unused)] let bob = pocketed_template.sender(bob_identity);
    #[allow(unused)] let carol = pocketed_template.sender(carol_identity);
    #[allow(unused)] let anonymous = pocketed_template.sender(anonymous_identity);

    // 🚩 1.1 permission permission_query
    assert_eq!(alice.version().unwrap(), 1_u32, "version");
    assert_eq!(alice.permission_all().unwrap(), vec![Forbidden("PauseQuery".to_string()), Permitted("PauseReplace".to_string()), Forbidden("PermissionQuery".to_string()), Permitted("PermissionFind".to_string()), Permitted("PermissionUpdate".to_string()), Permitted("RecordFind".to_string()), Permitted("RecordMigrate".to_string()), Permitted("ScheduleFind".to_string()), Permitted("ScheduleReplace".to_string()), Permitted("ScheduleTrigger".to_string()), Forbidden("BusinessExampleQuery".to_string()), Permitted("BusinessExampleSet".to_string())]);
    assert_eq!(alice.permission_query().unwrap(), vec!["PauseQuery", "PermissionQuery", "BusinessExampleQuery"].iter().map(|p| p.to_string()).collect::<Vec<_>>());
    assert_eq!(default.permission_query().unwrap(), vec!["PauseQuery", "PauseReplace", "PermissionQuery", "PermissionFind", "PermissionUpdate", "RecordFind", "RecordMigrate", "ScheduleFind", "ScheduleReplace", "ScheduleTrigger", "BusinessExampleQuery", "BusinessExampleSet"].iter().map(|p| p.to_string()).collect::<Vec<_>>());
    assert_eq!(bob.permission_update(vec![PermissionUpdatedArg::UpdateUserPermission(alice_identity, Some(vec!["PermissionUpdate".to_string(), "PermissionQuery".to_string()]))]).unwrap_err().reject_message, "Permission 'PermissionUpdate' is required".to_string());
    assert_eq!(default.permission_update(vec![PermissionUpdatedArg::UpdateUserPermission(alice_identity, Some(vec!["PermissionUpdate".to_string(), "PermissionQuery".to_string()]))]).unwrap(), ());
    assert_eq!(alice.permission_query().unwrap_err().reject_message, "Permission 'PermissionQuery' is required".to_string());
    assert_eq!(default.permission_query().unwrap(), vec!["PauseQuery", "PauseReplace", "PermissionQuery", "PermissionFind", "PermissionUpdate", "RecordFind", "RecordMigrate", "ScheduleFind", "ScheduleReplace", "ScheduleTrigger", "BusinessExampleQuery", "BusinessExampleSet"].iter().map(|p| p.to_string()).collect::<Vec<_>>());
    assert_eq!(default.permission_find_by_user(alice_identity).unwrap(), vec!["PauseQuery", "PermissionUpdate", "BusinessExampleQuery"].iter().map(|p| p.to_string()).collect::<Vec<_>>());
    assert_eq!(alice.permission_update(vec![PermissionUpdatedArg::UpdateUserPermission(alice_identity, None)]).unwrap(), ());
    assert_eq!(alice.permission_query().unwrap(), vec!["PauseQuery", "PermissionQuery", "BusinessExampleQuery"].iter().map(|p| p.to_string()).collect::<Vec<_>>());
    assert_eq!(default.permission_query().unwrap(), vec!["PauseQuery", "PauseReplace", "PermissionQuery", "PermissionFind", "PermissionUpdate", "RecordFind", "RecordMigrate", "ScheduleFind", "ScheduleReplace", "ScheduleTrigger", "BusinessExampleQuery", "BusinessExampleSet"].iter().map(|p| p.to_string()).collect::<Vec<_>>());

    // 🚩 1.2 permission permission update
    assert_eq!(default.permission_query().unwrap(), vec!["PauseQuery", "PauseReplace", "PermissionQuery", "PermissionFind", "PermissionUpdate", "RecordFind", "RecordMigrate", "ScheduleFind", "ScheduleReplace", "ScheduleTrigger", "BusinessExampleQuery", "BusinessExampleSet"].iter().map(|p| p.to_string()).collect::<Vec<_>>());
    assert_eq!(alice.permission_query().unwrap(), vec!["PauseQuery", "PermissionQuery", "BusinessExampleQuery"].iter().map(|p| p.to_string()).collect::<Vec<_>>());
    assert_eq!(default.permission_find_by_user(default_identity).unwrap(), vec!["PauseQuery", "PauseReplace", "PermissionQuery", "PermissionFind", "PermissionUpdate", "RecordFind", "RecordMigrate", "ScheduleFind", "ScheduleReplace", "ScheduleTrigger", "BusinessExampleQuery", "BusinessExampleSet"].iter().map(|p| p.to_string()).collect::<Vec<_>>());
    assert_eq!(default.permission_find_by_user(alice_identity).unwrap(), vec!["PauseQuery", "PermissionQuery", "BusinessExampleQuery"].iter().map(|p| p.to_string()).collect::<Vec<_>>());
    assert_eq!(alice.permission_find_by_user(default_identity).unwrap_err().reject_message, "Permission 'PermissionFind' is required".to_string());
    assert_eq!(alice.permission_find_by_user(alice_identity).unwrap_err().reject_message, "Permission 'PermissionFind' is required".to_string());

    // 🚩 1.3 permission roles
    assert_eq!(alice.permission_query().unwrap(), vec!["PauseQuery", "PermissionQuery", "BusinessExampleQuery"].iter().map(|p| p.to_string()).collect::<Vec<_>>());
    assert_eq!(default.permission_update(vec![PermissionUpdatedArg::UpdateRolePermission("Admin".to_string(), Some(vec!["PauseReplace".to_string(), "PauseQuery".to_string()]))]).unwrap(), ());
    assert_eq!(default.permission_update(vec![PermissionUpdatedArg::UpdateUserRole(alice_identity, Some(vec!["Admin".to_string()]))]).unwrap(), ());
    assert_eq!(alice.permission_query().unwrap(), vec!["PauseReplace", "PermissionQuery", "BusinessExampleQuery"].iter().map(|p| p.to_string()).collect::<Vec<_>>());
    assert_eq!(default.permission_update(vec![PermissionUpdatedArg::UpdateUserRole(alice_identity, None)]).unwrap(), ());
    assert_eq!(alice.permission_query().unwrap(), vec!["PauseQuery", "PermissionQuery", "BusinessExampleQuery"].iter().map(|p| p.to_string()).collect::<Vec<_>>());

    // 🚩 2.1 pause permission
    assert_eq!(default.pause_query().unwrap(), false);
    assert_eq!(default.pause_query_reason().unwrap(), None);
    assert_eq!(default.pause_replace(Some("reason".to_string())).unwrap(), ());
    assert_eq!(default.pause_query().unwrap(), true);
    assert_eq!(default.pause_query_reason().unwrap().unwrap().message, "reason".to_string());

    // 🚩 2.2 pause permission by alice
    assert_eq!(alice.pause_query().unwrap(), true);
    assert_eq!(alice.pause_query_reason().unwrap().unwrap().message, "reason".to_string());

    // 🚩 2.3 pause no permission
    assert_eq!(alice.pause_replace(None).unwrap_err().reject_message, "Permission 'PauseReplace' is required".to_string());
    assert_eq!(default.permission_update(vec![PermissionUpdatedArg::UpdateUserPermission(alice_identity, Some(vec!["PauseReplace".to_string(), "PauseQuery".to_string()]))]).unwrap(), ());
    assert_eq!(alice.pause_replace(None).unwrap(), ());
    assert_eq!(alice.pause_query().unwrap_err().reject_message, "Permission 'PauseQuery' is required".to_string());
    assert_eq!(alice.pause_query_reason().unwrap_err().reject_message, "Permission 'PauseQuery' is required".to_string());
    assert_eq!(default.pause_query().unwrap(), false);
    assert_eq!(default.pause_query_reason().unwrap(), None);

    // 🚩 3 record no permission
    assert_eq!(alice.record_topics().unwrap_err().reject_message, "Permission 'RecordFind' is required".to_string());
    assert_eq!(default.record_topics().unwrap(), ["Example", "CyclesCharge", "Upgrade", "Schedule", "Record", "Permission", "Pause", "Initial"].iter().map(|t| t.to_string()).collect::<Vec<_>>());
    let mut page_data = default.record_find_by_page(QueryPage { page: 1, size: 1 }, Some(RecordSearchArg{ id: None, created: None, topic: Some(vec!["Pause".to_string()]), content: None, caller: None })).unwrap();
    assert_eq!(page_data.total, 2);
    assert_eq!(page_data.page, 1);
    assert_eq!(page_data.size, 1);
    assert_eq!(page_data.data.len(), 1);
    assert_eq!(page_data.data.pop().unwrap().content.contains(r#"message: "reason" } -> None"#), true);
    assert_eq!(default.record_migrate(1).unwrap().removed, 0);

    // 🚩 4 schedule
    assert_eq!(alice.schedule_find().unwrap_err().reject_message, "Permission 'ScheduleFind' is required".to_string());
    assert_eq!(default.schedule_find().unwrap(), None);
    assert_eq!(alice.schedule_replace(Some(1000000000)).unwrap_err().reject_message, "Permission 'ScheduleReplace' is required".to_string());
    assert_eq!(default.schedule_replace(Some(1000000000)).unwrap(), ());
    std::thread::sleep(std::time::Duration::from_secs(3));
    assert_eq!(default.schedule_replace(None).unwrap(), ());
    std::thread::sleep(std::time::Duration::from_secs(2));
    assert_eq!(alice.schedule_trigger().unwrap_err().reject_message, "Permission 'ScheduleTrigger' is required".to_string());
    assert_eq!(default.schedule_trigger().unwrap(), ());

    // 🚩 5 example business
    assert_eq!(alice.business_example_query().unwrap(), "".to_string());
    assert_eq!(default.business_example_query().unwrap(), "".to_string());
    assert_eq!(alice.business_example_set("test string".to_string()).unwrap_err().reject_message, "Permission 'BusinessExampleSet' is required".to_string());
    assert_eq!(default.business_example_set("test string".to_string()).unwrap(), ());
    assert_eq!(alice.business_example_query().unwrap(), "test string".to_string());
    assert_eq!(default.business_example_query().unwrap(), "test string".to_string());

    // 🚩 6 test stable data
    assert_eq!(default.pause_replace(Some("reason".to_string())).unwrap(), ());
    assert_eq!(default.pause_query().unwrap(), true);
    pic.upgrade_canister(template, WASM_MODULE.to_vec(), encode_one(None::<()>).unwrap(), Some(default_identity)).unwrap();
    assert_eq!(default.pause_replace(None).unwrap(), ());
    assert_eq!(default.pause_query().unwrap(), false);
    assert_eq!(default.business_example_query().unwrap(), "test string".to_string());
}

mod interface {
    // This is an experimental feature to generate Rust binding from Candid.
    // You may want to manually adjust some of the types.
    #![allow(dead_code, unused_imports)]
    use candid::{
        self, CandidType, Decode, Deserialize, Encode, Principal, encode_args, encode_one, utils::ArgumentEncoder,
    };
    use pocket_ic::RejectResponse;
    use serde::de::DeserializeOwned;

    #[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
    pub struct InitArg {
        pub supers: Option<Vec<Principal>>,
        pub schedule: Option<candid::Nat>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
    pub enum InitArgs {
        V0(InitArg),
        V1(InitArg),
    }

    #[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
    pub struct MemoryMetrics {
        pub wasm_binary_size: candid::Nat,
        pub wasm_chunk_store_size: candid::Nat,
        pub canister_history_size: candid::Nat,
        pub stable_memory_size: candid::Nat,
        pub snapshots_size: candid::Nat,
        pub wasm_memory_size: candid::Nat,
        pub global_memory_size: candid::Nat,
        pub custom_sections_size: candid::Nat,
    }

    #[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
    pub enum CanisterStatusType {
        #[serde(rename = "stopped")]
        Stopped,
        #[serde(rename = "stopping")]
        Stopping,
        #[serde(rename = "running")]
        Running,
    }

    #[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
    pub enum LogVisibility {
        #[serde(rename = "controllers")]
        Controllers,
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "allowed_viewers")]
        AllowedViewers(Vec<Principal>),
    }

    #[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
    pub struct DefiniteCanisterSettings {
        pub freezing_threshold: candid::Nat,
        pub wasm_memory_threshold: candid::Nat,
        pub controllers: Vec<Principal>,
        pub reserved_cycles_limit: candid::Nat,
        pub log_visibility: LogVisibility,
        pub wasm_memory_limit: candid::Nat,
        pub memory_allocation: candid::Nat,
        pub compute_allocation: candid::Nat,
    }

    #[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
    pub struct QueryStats {
        pub response_payload_bytes_total: candid::Nat,
        pub num_instructions_total: candid::Nat,
        pub num_calls_total: candid::Nat,
        pub request_payload_bytes_total: candid::Nat,
    }

    #[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
    pub struct CanisterStatusResult {
        pub memory_metrics: MemoryMetrics,
        pub status: CanisterStatusType,
        pub memory_size: candid::Nat,
        pub cycles: candid::Nat,
        pub settings: DefiniteCanisterSettings,
        pub query_stats: QueryStats,
        pub idle_cycles_burned_per_day: candid::Nat,
        pub module_hash: Option<serde_bytes::ByteBuf>,
        pub reserved_cycles: candid::Nat,
    }

    #[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
    pub struct PauseReason {
        pub timestamp_nanos: candid::Int,
        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
    pub enum Permission {
        Permitted(String),
        Forbidden(String),
    }

    #[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
    pub enum PermissionUpdatedArg {
        UpdateRolePermission(String, Option<Vec<String>>),
        UpdateUserPermission(Principal, Option<Vec<String>>),
        UpdateUserRole(Principal, Option<Vec<String>>),
    }

    #[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
    pub struct QueryPage {
        pub page: u64,
        pub size: u32,
    }

    #[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
    pub struct RecordSearchArg {
        pub id: Option<(Option<u64>, Option<u64>)>,
        pub created: Option<(Option<u64>, Option<u64>)>,
        pub topic: Option<Vec<String>>,
        pub content: Option<String>,
        pub caller: Option<Vec<Principal>>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
    pub struct Record {
        pub id: u64,
        pub created: candid::Int,
        pub topic: u8,
        pub content: String,
        pub done: Option<(candid::Int, String)>,
        pub caller: Principal,
    }

    #[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
    pub struct PageData {
        pub total: u64,
        pub data: Vec<Record>,
        pub page: u64,
        pub size: u32,
    }

    #[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
    pub struct MigratedRecords {
        pub records: Vec<Record>,
        pub next_id: u64,
        pub removed: u64,
    }

    #[derive(Clone, Copy)]
    pub struct PocketedCanisterId<'a> {
        pub(super) canister_id: Principal,
        pub(super) pic: &'a pocket_ic::PocketIc,
    }

    impl<'a> PocketedCanisterId<'a> {
        pub fn new(canister_id: Principal, pic: &'a pocket_ic::PocketIc) -> Self {
            Self { canister_id, pic }
        }
        pub fn sender(&self, sender: Principal) -> Service<'a> {
            Service { pocket: *self, sender }
        }
    }

    type Result<R> = std::result::Result<R, RejectResponse>;
    pub struct Service<'a> {
        pub(super) pocket: PocketedCanisterId<'a>,
        pub(super) sender: Principal,
    }
    impl Service<'_> {
        fn query_call<R: CandidType + DeserializeOwned>(&self, method: &str, payload: Vec<u8>) -> Result<R> {
            let response = self
                .pocket
                .pic
                .query_call(self.pocket.canister_id, self.sender, method, payload)?;
            let result = Decode!(response.as_slice(), R).unwrap();
            Ok(result)
        }
        fn update_call<R: CandidType + DeserializeOwned>(&self, method: &str, payload: Vec<u8>) -> Result<R> {
            let response = self
                .pocket
                .pic
                .update_call(self.pocket.canister_id, self.sender, method, payload)?;
            let result = Decode!(response.as_slice(), R).unwrap();
            Ok(result)
        }
        pub fn get_candid_interface_tmp_hack(&self) -> Result<String> {
            self.query_call("__get_candid_interface_tmp_hack", Encode!(&()).unwrap())
        }
        pub fn business_example_query(&self) -> Result<String> {
            self.query_call("business_example_query", Encode!(&()).unwrap())
        }
        pub fn business_example_set(&self, arg0: String) -> Result<()> {
            self.update_call("business_example_set", encode_one(&arg0).unwrap())
        }
        pub fn canister_status(&self) -> Result<CanisterStatusResult> {
            self.update_call("canister_status", Encode!(&()).unwrap())
        }
        pub fn pause_query(&self) -> Result<bool> {
            self.query_call("pause_query", Encode!(&()).unwrap())
        }
        pub fn pause_query_reason(&self) -> Result<Option<PauseReason>> {
            self.query_call("pause_query_reason", Encode!(&()).unwrap())
        }
        pub fn pause_replace(&self, arg0: Option<String>) -> Result<()> {
            self.update_call("pause_replace", encode_one(&arg0).unwrap())
        }
        pub fn permission_all(&self) -> Result<Vec<Permission>> {
            self.query_call("permission_all", Encode!(&()).unwrap())
        }
        pub fn permission_assigned_by_user(&self, arg0: Principal) -> Result<Option<Vec<Permission>>> {
            self.query_call("permission_assigned_by_user", encode_one(&arg0).unwrap())
        }
        pub fn permission_assigned_query(&self) -> Result<Option<Vec<Permission>>> {
            self.query_call("permission_assigned_query", Encode!(&()).unwrap())
        }
        pub fn permission_find_by_user(&self, arg0: Principal) -> Result<Vec<String>> {
            self.query_call("permission_find_by_user", encode_one(&arg0).unwrap())
        }
        pub fn permission_query(&self) -> Result<Vec<String>> {
            self.query_call("permission_query", Encode!(&()).unwrap())
        }
        pub fn permission_roles_all(&self) -> Result<Vec<(String, Vec<Permission>)>> {
            self.query_call("permission_roles_all", Encode!(&()).unwrap())
        }
        pub fn permission_roles_by_user(&self, arg0: Principal) -> Result<Option<Vec<String>>> {
            self.query_call("permission_roles_by_user", encode_one(&arg0).unwrap())
        }
        pub fn permission_roles_query(&self) -> Result<Option<Vec<String>>> {
            self.query_call("permission_roles_query", Encode!(&()).unwrap())
        }
        pub fn permission_update(&self, arg0: Vec<PermissionUpdatedArg>) -> Result<()> {
            self.update_call("permission_update", encode_one(&arg0).unwrap())
        }
        pub fn record_find_by_page(&self, arg0: QueryPage, arg1: Option<RecordSearchArg>) -> Result<PageData> {
            self.query_call("record_find_by_page", encode_args((&arg0, &arg1)).unwrap())
        }
        pub fn record_migrate(&self, arg0: u32) -> Result<MigratedRecords> {
            self.update_call("record_migrate", encode_one(&arg0).unwrap())
        }
        pub fn record_topics(&self) -> Result<Vec<String>> {
            self.query_call("record_topics", Encode!(&()).unwrap())
        }
        pub fn schedule_find(&self) -> Result<Option<u64>> {
            self.query_call("schedule_find", Encode!(&()).unwrap())
        }
        pub fn schedule_replace(&self, arg0: Option<u64>) -> Result<()> {
            self.update_call("schedule_replace", encode_one(&arg0).unwrap())
        }
        pub fn schedule_trigger(&self) -> Result<()> {
            self.update_call("schedule_trigger", Encode!(&()).unwrap())
        }
        pub fn version(&self) -> Result<u32> {
            self.query_call("version", Encode!(&()).unwrap())
        }
        pub fn wallet_balance(&self) -> Result<candid::Nat> {
            self.query_call("wallet_balance", Encode!(&()).unwrap())
        }
        pub fn wallet_receive(&self) -> Result<candid::Nat> {
            self.query_call("wallet_receive", Encode!(&()).unwrap())
        }
        pub fn whoami(&self) -> Result<Principal> {
            self.query_call("whoami", Encode!(&()).unwrap())
        }
    }
}
