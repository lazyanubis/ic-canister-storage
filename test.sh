#!/usr/bin/env bash
start_time=$(date +%H:%M:%S)
start_time_s=$(date +%s)

# 运行完毕自动停止
dfx stop
trap 'say test over && dfx stop' EXIT

dfx start --background --clean # 开启新的 dfx 环境
# dfx start --background --clean >/dev/null 2>&1 # 开启新的 dfx 环境

function red { echo "\033[31m$1\033[0m"; }
function green { echo "\033[32m$1\033[0m"; }
function yellow { echo "\033[33m$1\033[0m"; }
function blue { echo "\033[34m$1\033[0m"; }

function canister_id {
    # cat ".dfx/local/canister_ids.json"
    # echo $(cat ".dfx/local/canister_ids.json" | tr -d '\n' | awk -F "$1" '{print $2}' | awk -F "\": \"" '{print $2}' | awk -F "\"" '{print $1}')
    echo $(dfx canister id $1)
}

function check {
    if [ -n "$3" ]; then
        if [[ $(echo $2 | grep "$3") != "" ]]; then
            green "✅ Passed: $1 -> $2 -> $3"
        else
            red "❌ Failed: $1"
            green "Expected: $3"
            yellow "Got: $2"
            red "Line: ./test.sh:$5 👉 $4"
            exit 1
        fi
    fi
}

function test {
    tips="$1"
    result="$(echo $2 | tr -d '\n')"
    check "$tips" "$result" "$3" "1" "$(caller 0)"
    check "$tips" "$result" "$4" "2" "$(caller 0)"
    check "$tips" "$result" "$5" "3" "$(caller 0)"
    check "$tips" "$result" "$6" "4" "$(caller 0)"
    check "$tips" "$result" "$7" "5" "$(caller 0)"
    check "$tips" "$result" "$8" "6" "$(caller 0)"
    check "$tips" "$result" "$9" "7" "$(caller 0)"
}

ANONYMOUS="2vxsx-fae"
DEFAULT=$(dfx identity get-principal)
ALICE=$(dfx --identity alice identity get-principal)
BOB=$(dfx --identity bob identity get-principal)

# cargo test
cargo clippy
# cargo audit --no-fetch --quiet

# ! 1. 测试 service
red "\n=========== 1. service ===========\n"
dfx canister create service --specified-id "bkyz2-fmaaa-aaaaa-qaaaq-cai" # --with-cycles 50T
dfx deploy --mode=reinstall --yes --argument "(null)" service
service=$(canister_id "service")
blue "Service Canister: $service"

if [ -z "$service" ]; then
    say deploy failed
    exit 1
fi

blue "\n🚩 1.1 permission permission_query"
test "version" "$(dfx --identity alice canister call service version 2>&1)" '(1 : nat32)'
test "permission_all" "$(dfx --identity alice canister call service permission_all 2>&1)" 'vec { variant { Forbidden = "PauseQuery" }; variant { Permitted = "PauseReplace" }'
test "permission_query" "$(dfx --identity alice canister call service permission_query 2>&1)" '(vec { "PauseQuery"; "PermissionQuery"; "BusinessExampleQuery" })'
test "permission_query" "$(dfx canister call service permission_query 2>&1)" 'vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessExampleQuery"; "BusinessExampleSet";}'
test "permission_update" "$(dfx --identity bob canister call service permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; opt vec { \"PermissionUpdate\";\"PermissionQuery\" } } } })" 2>&1)" "'PermissionUpdate' is required"
test "permission_update" "$(dfx canister call service permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; opt vec { \"PermissionUpdate\";\"PermissionQuery\" } } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call service permission_query 2>&1)" "'PermissionQuery' is required"
test "permission_query" "$(dfx canister call service permission_query 2>&1)" 'vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessExampleQuery"; "BusinessExampleSet";}'
test "permission_find_by_user" "$(dfx canister call service permission_find_by_user "(principal \"$ALICE\")" 2>&1)" '(vec { "PauseQuery"; "PermissionUpdate"; "BusinessExampleQuery" })'
test "permission_update" "$(dfx --identity alice canister call service permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; null } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call service permission_query 2>&1)" '(vec { "PauseQuery"; "PermissionQuery"; "BusinessExampleQuery" })'
test "permission_query" "$(dfx canister call service permission_query 2>&1)" 'vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessExampleQuery"; "BusinessExampleSet";}'

blue "\n🚩 1.2 permission permission update"
test "permission_query" "$(dfx canister call service permission_query 2>&1)" 'vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessExampleQuery"; "BusinessExampleSet";}'
test "permission_query" "$(dfx --identity alice canister call service permission_query 2>&1)" 'vec { "PauseQuery"; "PermissionQuery"; "BusinessExampleQuery" }'
test "permission_find_by_user" "$(dfx canister call service permission_find_by_user "(principal \"$DEFAULT\")" 2>&1)" 'vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessExampleQuery"; "BusinessExampleSet";}'
test "permission_find_by_user" "$(dfx canister call service permission_find_by_user "(principal \"$ALICE\")" 2>&1)" 'vec { "PauseQuery"; "PermissionQuery"; "BusinessExampleQuery" }'
test "permission_find_by_user" "$(dfx --identity alice canister call service permission_find_by_user "(principal \"$DEFAULT\")" 2>&1)" "'PermissionFind' is required"
test "permission_find_by_user" "$(dfx --identity alice canister call service permission_find_by_user "(principal \"$ALICE\")" 2>&1)" "'PermissionFind' is required"

blue "\n🚩 1.3 permission roles"
test "permission_query" "$(dfx --identity alice canister call service permission_query 2>&1)" 'vec { "PauseQuery"; "PermissionQuery"; "BusinessExampleQuery" }'
test "permission_update" "$(dfx canister call service permission_update "(vec { variant { UpdateRolePermission=record{\"Admin\"; opt vec {\"PauseReplace\"; \"PauseQuery\"} } } })" 2>&1)" "()"
test "permission_update" "$(dfx canister call service permission_update "(vec { variant { UpdateUserRole=record{principal \"$ALICE\"; opt vec {\"Admin\"} } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call service permission_query 2>&1)" '(vec { "PauseReplace"; "PermissionQuery"; "BusinessExampleQuery" })'
test "permission_update" "$(dfx canister call service permission_update "(vec { variant { UpdateUserRole=record{principal \"$ALICE\"; null } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call service permission_query 2>&1)" 'vec { "PauseQuery"; "PermissionQuery"; "BusinessExampleQuery" }'

blue "\n🚩 2.1 pause permission"
test "pause_query" "$(dfx canister call service pause_query 2>&1)" "(false)"
test "pause_query_reason" "$(dfx canister call service pause_query_reason 2>&1)" "(null)"
test "pause_replace" "$(dfx canister call service pause_replace "(opt \"reason\")" 2>&1)" "()"
test "pause_query" "$(dfx canister call service pause_query 2>&1)" "(true)"
test "pause_query_reason" "$(dfx canister call service pause_query_reason 2>&1)" "message = \"reason\""

blue "\n🚩 2.2 pause permission by alice"
test "pause_query" "$(dfx --identity alice canister call service pause_query 2>&1)" "(true)"
test "pause_query_reason" "$(dfx --identity alice canister call service pause_query_reason 2>&1)" "message = \"reason\""

blue "\n🚩 2.3 pause no permission"
test "pause_replace" "$(dfx --identity alice canister call service pause_replace "(null)" 2>&1)" "'PauseReplace' is required"
test "permission_update" "$(dfx canister call service permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; opt vec { \"PauseReplace\";\"PauseQuery\" } } } })" 2>&1)" "()"
test "pause_replace" "$(dfx --identity alice canister call service pause_replace "(null)" 2>&1)" "()"
test "pause_query" "$(dfx --identity alice canister call service pause_query 2>&1)" "'PauseQuery' is required"
test "pause_query_reason" "$(dfx --identity alice canister call service pause_query_reason 2>&1)" "'PauseQuery' is required"
test "pause_query" "$(dfx canister call service pause_query 2>&1)" "(false)"
test "pause_query_reason" "$(dfx canister call service pause_query_reason 2>&1)" "(null)"

blue "\n🚩 3 record no permission"
test "record_topics" "$(dfx --identity alice canister call service record_topics 2>&1)" "'RecordFind' is required"
test "record_topics" "$(dfx canister call service record_topics 2>&1)" '"Example"' '"CyclesCharge"'
test "record_find_by_page" "$(dfx canister call service record_find_by_page "(record{page=1:nat64;size=1:nat32},opt record{topic=opt vec{\"Pause\"}})" 2>&1)" "record { total = "
test "record_migrate" "$(dfx canister call service record_migrate "(1:nat32)" 2>&1)" "removed = 0"

blue "\n🚩 4 schedule"
test "schedule_find" "$(dfx --identity alice canister call service schedule_find 2>&1)" "'ScheduleFind' is required"
test "schedule_find" "$(dfx canister call service schedule_find 2>&1)" "(null)"
test "schedule_replace" "$(dfx --identity alice canister call service schedule_replace "(opt (1000000000:nat64))" 2>&1)" "'ScheduleReplace' is required"
test "schedule_replace" "$(dfx canister call service schedule_replace "(opt (1000000000:nat64))" 2>&1)" "()"
sleep 3
test "schedule_replace" "$(dfx canister call service schedule_replace "(null)" 2>&1)" "()"
sleep 2
test "schedule_trigger" "$(dfx --identity alice canister call service schedule_trigger 2>&1)" "'ScheduleTrigger' is required"
test "schedule_trigger" "$(dfx canister call service schedule_trigger 2>&1)" "()"

blue "\n🚩 5 example business"
test "business_example_query" "$(dfx --identity alice canister call service business_example_query 2>&1)" "\"\""
test "business_example_query" "$(dfx canister call service business_example_query 2>&1)" "\"\""
test "business_example_set" "$(dfx --identity alice canister call service business_example_set "(\"test string\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_set" "$(dfx canister call service business_example_set "(\"test string\")" 2>&1)" "()"
test "business_example_query" "$(dfx --identity alice canister call service business_example_query 2>&1)" "test string"
test "business_example_query" "$(dfx canister call service business_example_query 2>&1)" "test string"

blue "\n🚩 6 test service data"
test "pause_replace" "$(dfx canister call service pause_replace "(opt \"reason\")" 2>&1)" "()"
test "pause_query" "$(dfx canister call service pause_query 2>&1)" "(true)"
dfx canister install --mode=upgrade --upgrade-unchanged --argument "(null)" service
test "pause_replace" "$(dfx canister call service pause_replace "(null)" 2>&1)" "()"
test "pause_query" "$(dfx canister call service pause_query 2>&1)" "(false)"
test "business_example_query" "$(dfx canister call service business_example_query 2>&1)" "test string"

blue "\n🚩 7 test service cell"
test "business_example_cell_query" "$(dfx --identity alice canister call service business_example_cell_query 2>&1)" "\"\""
test "business_example_cell_query" "$(dfx canister call service business_example_cell_query 2>&1)" "\"\""
test "business_example_cell_set" "$(dfx --identity alice canister call service business_example_cell_set "(\"test string\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_cell_set" "$(dfx canister call service business_example_cell_set "(\"test string\")" 2>&1)" "()"
test "business_example_cell_query" "$(dfx --identity alice canister call service business_example_cell_query 2>&1)" "test string"
test "business_example_cell_query" "$(dfx canister call service business_example_cell_query 2>&1)" "test string"

blue "\n🚩 8 test service vec"
test "business_example_vec_query" "$(dfx --identity alice canister call service business_example_vec_query 2>&1)" "(vec {})"
test "business_example_vec_query" "$(dfx canister call service business_example_vec_query 2>&1)" "(vec {})"
test "business_example_vec_pop" "$(dfx --identity alice canister call service business_example_vec_pop "()" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_vec_pop" "$(dfx canister call service business_example_vec_pop "()" 2>&1)" "(null)"
test "business_example_vec_push" "$(dfx --identity alice canister call service business_example_vec_push "(5: nat64)" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_vec_push" "$(dfx canister call service business_example_vec_push "(5: nat64)" 2>&1)" "()"
test "business_example_vec_query" "$(dfx --identity alice canister call service business_example_vec_query 2>&1)" "(vec { record { vec_data = 5 : nat64 } })"
test "business_example_vec_query" "$(dfx canister call service business_example_vec_query 2>&1)" "(vec { record { vec_data = 5 : nat64 } })"
test "business_example_vec_pop" "$(dfx --identity alice canister call service business_example_vec_pop "()" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_vec_pop" "$(dfx canister call service business_example_vec_pop "()" 2>&1)" "(opt record { vec_data = 5 : nat64 })"
test "business_example_vec_query" "$(dfx --identity alice canister call service business_example_vec_query 2>&1)" "(vec {})"
test "business_example_vec_query" "$(dfx canister call service business_example_vec_query 2>&1)" "(vec {})"

blue "\n🚩 9 test service map"
test "business_example_map_query" "$(dfx --identity alice canister call service business_example_map_query 2>&1)" "(vec {})"
test "business_example_map_query" "$(dfx canister call service business_example_map_query 2>&1)" "(vec {})"
test "business_example_map_update" "$(dfx --identity alice canister call service business_example_map_update "(1:nat64, opt \"111\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_map_update" "$(dfx canister call service business_example_map_update "(1:nat64, opt \"111\")" 2>&1)" "(null)"
test "business_example_map_query" "$(dfx --identity alice canister call service business_example_map_query 2>&1)" '(vec { record { 1 : nat64; "111" } })'
test "business_example_map_query" "$(dfx canister call service business_example_map_query 2>&1)" '(vec { record { 1 : nat64; "111" } })'
test "business_example_map_update" "$(dfx canister call service business_example_map_update "(1:nat64, opt \"123\")" 2>&1)" "(opt \"111\")"
test "business_example_map_update" "$(dfx canister call service business_example_map_update "(1:nat64, null)" 2>&1)" "(opt \"123\")"
test "business_example_map_update" "$(dfx canister call service business_example_map_update "(2:nat64, opt \"222\")" 2>&1)" "(null)"
test "business_example_map_query" "$(dfx --identity alice canister call service business_example_map_query 2>&1)" '(vec { record { 2 : nat64; "222" } })'
test "business_example_map_query" "$(dfx canister call service business_example_map_query 2>&1)" '(vec { record { 2 : nat64; "222" } })'

blue "\n🚩 10 test service log"
test "business_example_log_query" "$(dfx --identity alice canister call service business_example_log_query 2>&1)" "(vec {})"
test "business_example_log_query" "$(dfx canister call service business_example_log_query 2>&1)" "(vec {})"
test "business_example_log_update" "$(dfx --identity alice canister call service business_example_log_update "(\"111\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_log_update" "$(dfx canister call service business_example_log_update "(\"111\")" 2>&1)" "(0 : nat64)"
test "business_example_log_query" "$(dfx --identity alice canister call service business_example_log_query 2>&1)" '(vec { "111" })'
test "business_example_log_query" "$(dfx canister call service business_example_log_query 2>&1)" '(vec { "111" })'
test "business_example_log_update" "$(dfx canister call service business_example_log_update "(\"123\")" 2>&1)" "(1 : nat64)"
test "business_example_log_query" "$(dfx --identity alice canister call service business_example_log_query 2>&1)" '(vec { "111"; "123" })'
test "business_example_log_query" "$(dfx canister call service business_example_log_query 2>&1)" '(vec { "111"; "123" })'

blue "\n🚩 11 test service priority queue"
test "business_example_priority_queue_query" "$(dfx --identity alice canister call service business_example_priority_queue_query 2>&1)" "(vec {})"
test "business_example_priority_queue_query" "$(dfx canister call service business_example_priority_queue_query 2>&1)" "(vec {})"
test "business_example_priority_queue_pop" "$(dfx --identity alice canister call service business_example_priority_queue_pop "()" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_priority_queue_pop" "$(dfx canister call service business_example_priority_queue_pop "()" 2>&1)" "(null)"
test "business_example_priority_queue_push" "$(dfx --identity alice canister call service business_example_priority_queue_push "(5: nat64)" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_priority_queue_push" "$(dfx canister call service business_example_priority_queue_push "(5: nat64)" 2>&1)" "()"
test "business_example_priority_queue_query" "$(dfx --identity alice canister call service business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"
test "business_example_priority_queue_query" "$(dfx canister call service business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"
test "business_example_priority_queue_push" "$(dfx canister call service business_example_priority_queue_push "(2: nat64)" 2>&1)" "()"
test "business_example_priority_queue_query" "$(dfx canister call service business_example_priority_queue_query 2>&1)" "(vec { 2 : nat64; 5 : nat64 })"
test "business_example_priority_queue_pop" "$(dfx --identity alice canister call service business_example_priority_queue_pop "()" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_priority_queue_pop" "$(dfx canister call service business_example_priority_queue_pop "()" 2>&1)" "(opt (2 : nat64))"
test "business_example_priority_queue_query" "$(dfx --identity alice canister call service business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"
test "business_example_priority_queue_query" "$(dfx canister call service business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"

blue "\n🚩 12 test service priority queue"
test "pause_replace" "$(dfx canister call service pause_replace "(opt \"reason\")" 2>&1)" "()"
test "pause_query" "$(dfx canister call service pause_query 2>&1)" "(true)"
dfx canister install --mode=upgrade --upgrade-unchanged --argument "(null)" service
test "pause_replace" "$(dfx canister call service pause_replace "(null)" 2>&1)" "()"
test "pause_query" "$(dfx canister call service pause_query 2>&1)" "(false)"
test "business_example_query" "$(dfx canister call service business_example_query 2>&1)" "test string"
test "business_example_cell_query" "$(dfx --identity alice canister call service business_example_cell_query 2>&1)" "test string"
test "business_example_cell_query" "$(dfx canister call service business_example_cell_query 2>&1)" "test string"
test "business_example_vec_query" "$(dfx --identity alice canister call service business_example_vec_query 2>&1)" "(vec {})"
test "business_example_vec_query" "$(dfx canister call service business_example_vec_query 2>&1)" "(vec {})"
test "business_example_map_query" "$(dfx --identity alice canister call service business_example_map_query 2>&1)" '(vec { record { 2 : nat64; "222" } })'
test "business_example_map_query" "$(dfx canister call service business_example_map_query 2>&1)" '(vec { record { 2 : nat64; "222" } })'
test "business_example_log_query" "$(dfx --identity alice canister call service business_example_log_query 2>&1)" '(vec { "111"; "123" })'
test "business_example_log_query" "$(dfx canister call service business_example_log_query 2>&1)" '(vec { "111"; "123" })'
test "business_example_priority_queue_query" "$(dfx --identity alice canister call service business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"
test "business_example_priority_queue_query" "$(dfx canister call service business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"

# test completed

green "\n=================== TEST COMPLETED AND SUCCESSFUL ===================\n"

end_time=$(date +%H:%M:%S)
end_time_s=$(date +%s)
spend=$(($end_time_s - $start_time_s))
spend_minutes=$(($spend / 60))
echo "✅ $start_time -> $end_time" "Total: $spend seconds ($spend_minutes mins) 🎉🎉🎉\n"

say test successful

# sleep 10000
# read -s -n1 -p "按任意键结束..."
