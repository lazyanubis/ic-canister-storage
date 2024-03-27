#!/usr/bin/env bash

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

function test {
    tips="$1"
    result="$(echo $2 | tr -d '\n')"
    need1="$3"
    need2="$4"
    need3="$5"
    # echo $result
    # echo $need1
    # echo $need2
    # echo $need3
    if [[ $(echo $result | grep "$need1") != "" ]]; then
        green "* Passed: $tips -> $result -> $need1"
    else
        red "* Failed: $tips"
        green "Expected: $need1"
        yellow "Got: $result"
        exit 1
    fi
    if [[ $need2 != "" ]]; then
        if [[ $(echo $result | grep "$need2") != "" ]]; then
            green "* Passed: $tips -> $result -> $need2"
        else
            red "* Failed: $tips"
            green "Expected: $need2"
            yellow "Got: $result"
            exit 1
        fi
    fi
    if [[ $need3 != "" ]]; then
        if [[ $(echo $result | grep "$need3") != "" ]]; then
            green "* Passed: $tips -> $result -> $need3"
        else
            red "* Failed: $tips"
            green "Expected: $need3"
            yellow "Got: $result"
            exit 1
        fi
    fi
}

ANONYMOUS="2vxsx-fae"
DEFAULT=$(dfx identity get-principal)
ALICE=$(dfx --identity alice identity get-principal)
BOB=$(dfx --identity bob identity get-principal)

cargo test
cargo clippy
# cargo audit --no-fetch --quiet

# ! 1. 测试 template
red "\n=========== 1. template ===========\n"
dfx canister create template
dfx deploy --mode=reinstall --yes --argument "(null)" template
template=$(canister_id "template")
blue "Template Canister: $template"

if [ -z "$template" ]; then
    say deploy failed
    exit 1
fi

blue "1.1 permission permission_query"
test "version" "$(dfx --identity alice canister call template version 2>&1)" '(1 : nat32)'
test "permission_all" "$(dfx --identity alice canister call template permission_all 2>&1)" 'vec { variant { Forbidden = "PauseQuery" }; variant { Permitted = "PauseReplace" }'
test "permission_query" "$(dfx --identity alice canister call template permission_query 2>&1)" '(vec { "PauseQuery"; "PermissionQuery"; "BusinessExampleQuery" })'
test "permission_query" "$(dfx canister call template permission_query 2>&1)" 'vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessExampleQuery"; "BusinessExampleSet";}'
test "permission_update" "$(dfx --identity bob canister call template permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; opt vec { \"PermissionUpdate\";\"PermissionQuery\" } } } })" 2>&1)" "'PermissionUpdate' is required"
test "permission_update" "$(dfx canister call template permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; opt vec { \"PermissionUpdate\";\"PermissionQuery\" } } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call template permission_query 2>&1)" "'PermissionQuery' is required"
test "permission_query" "$(dfx canister call template permission_query 2>&1)" 'vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessExampleQuery"; "BusinessExampleSet";}'
test "permission_find_by_user" "$(dfx canister call template permission_find_by_user "(principal \"$ALICE\")" 2>&1)" '(vec { "PauseQuery"; "PermissionUpdate"; "BusinessExampleQuery" })'
test "permission_update" "$(dfx --identity alice canister call template permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; null } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call template permission_query 2>&1)" '(vec { "PauseQuery"; "PermissionQuery"; "BusinessExampleQuery" })'
test "permission_query" "$(dfx canister call template permission_query 2>&1)" 'vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessExampleQuery"; "BusinessExampleSet";}'

blue "1.2 permission permission update"
test "permission_query" "$(dfx canister call template permission_query 2>&1)" 'vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessExampleQuery"; "BusinessExampleSet";}'
test "permission_query" "$(dfx --identity alice canister call template permission_query 2>&1)" 'vec { "PauseQuery"; "PermissionQuery"; "BusinessExampleQuery" }'
test "permission_find_by_user" "$(dfx canister call template permission_find_by_user "(principal \"$DEFAULT\")" 2>&1)" 'vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessExampleQuery"; "BusinessExampleSet";}'
test "permission_find_by_user" "$(dfx canister call template permission_find_by_user "(principal \"$ALICE\")" 2>&1)" 'vec { "PauseQuery"; "PermissionQuery"; "BusinessExampleQuery" }'
test "permission_find_by_user" "$(dfx --identity alice canister call template permission_find_by_user "(principal \"$DEFAULT\")" 2>&1)" "'PermissionFind' is required"
test "permission_find_by_user" "$(dfx --identity alice canister call template permission_find_by_user "(principal \"$ALICE\")" 2>&1)" "'PermissionFind' is required"

blue "1.3 permission roles"
test "permission_query" "$(dfx --identity alice canister call template permission_query 2>&1)" 'vec { "PauseQuery"; "PermissionQuery"; "BusinessExampleQuery" }'
test "permission_update" "$(dfx canister call template permission_update "(vec { variant { UpdateRolePermission=record{\"Admin\"; opt vec {\"PauseReplace\"; \"PauseQuery\"} } } })" 2>&1)" "()"
test "permission_update" "$(dfx canister call template permission_update "(vec { variant { UpdateUserRole=record{principal \"$ALICE\"; opt vec {\"Admin\"} } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call template permission_query 2>&1)" '(vec { "PauseReplace"; "PermissionQuery"; "BusinessExampleQuery" })'
test "permission_update" "$(dfx canister call template permission_update "(vec { variant { UpdateUserRole=record{principal \"$ALICE\"; null } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call template permission_query 2>&1)" 'vec { "PauseQuery"; "PermissionQuery"; "BusinessExampleQuery" }'

blue "2.1 pause permission"
test "pause_query" "$(dfx canister call template pause_query 2>&1)" "(false)"
test "pause_query_reason" "$(dfx canister call template pause_query_reason 2>&1)" "(null)"
test "pause_replace" "$(dfx canister call template pause_replace "(opt \"reason\")" 2>&1)" "()"
test "pause_query" "$(dfx canister call template pause_query 2>&1)" "(true)"
test "pause_query_reason" "$(dfx canister call template pause_query_reason 2>&1)" "message = \"reason\""

blue "2.2 pause permission by alice"
test "pause_query" "$(dfx --identity alice canister call template pause_query 2>&1)" "(true)"
test "pause_query_reason" "$(dfx --identity alice canister call template pause_query_reason 2>&1)" "message = \"reason\""

blue "2.3 pause no permission"
test "pause_replace" "$(dfx --identity alice canister call template pause_replace "(null)" 2>&1)" "'PauseReplace' is required"
test "permission_update" "$(dfx canister call template permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; opt vec { \"PauseReplace\";\"PauseQuery\" } } } })" 2>&1)" "()"
test "pause_replace" "$(dfx --identity alice canister call template pause_replace "(null)" 2>&1)" "()"
test "pause_query" "$(dfx --identity alice canister call template pause_query 2>&1)" "'PauseQuery' is required"
test "pause_query_reason" "$(dfx --identity alice canister call template pause_query_reason 2>&1)" "'PauseQuery' is required"
test "pause_query" "$(dfx canister call template pause_query 2>&1)" "(false)"
test "pause_query_reason" "$(dfx canister call template pause_query_reason 2>&1)" "(null)"

blue "3 record no permission"
test "record_topics" "$(dfx --identity alice canister call template record_topics 2>&1)" "'RecordFind' is required"
test "record_topics" "$(dfx canister call template record_topics 2>&1)" '"Example"' '"CyclesCharge"'
test "record_find_by_page" "$(dfx canister call template record_find_by_page "(record{page=1:nat64;size=1:nat32},opt record{topic=opt vec{\"Pause\"}})" 2>&1)" "record { total = "
test "record_migrate" "$(dfx canister call template record_migrate "(1:nat32)" 2>&1)" "removed = 0"

blue "4 schedule"
test "schedule_find" "$(dfx --identity alice canister call template schedule_find 2>&1)" "'ScheduleFind' is required"
test "schedule_find" "$(dfx canister call template schedule_find 2>&1)" "(null)"
test "schedule_replace" "$(dfx --identity alice canister call template schedule_replace "(opt (1000000000:nat64))" 2>&1)" "'ScheduleReplace' is required"
test "schedule_replace" "$(dfx canister call template schedule_replace "(opt (1000000000:nat64))" 2>&1)" "()"
sleep 3
test "schedule_replace" "$(dfx canister call template schedule_replace "(null)" 2>&1)" "()"
sleep 2
test "schedule_trigger" "$(dfx --identity alice canister call template schedule_trigger 2>&1)" "'ScheduleTrigger' is required"
test "schedule_trigger" "$(dfx canister call template schedule_trigger 2>&1)" "()"

blue "5 example business"
test "business_example_query" "$(dfx --identity alice canister call template business_example_query 2>&1)" "\"\""
test "business_example_query" "$(dfx canister call template business_example_query 2>&1)" "\"\""
test "business_example_set" "$(dfx --identity alice canister call template business_example_set "(\"test string\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_set" "$(dfx canister call template business_example_set "(\"test string\")" 2>&1)" "()"
test "business_example_query" "$(dfx --identity alice canister call template business_example_query 2>&1)" "test string"
test "business_example_query" "$(dfx canister call template business_example_query 2>&1)" "test string"

blue "6 test stable data"
test "pause_replace" "$(dfx canister call template pause_replace "(opt \"reason\")" 2>&1)" "()"
test "pause_query" "$(dfx canister call template pause_query 2>&1)" "(true)"
dfx canister install --mode=upgrade --upgrade-unchanged --argument "(null)" template
test "pause_replace" "$(dfx canister call template pause_replace "(null)" 2>&1)" "()"
test "pause_query" "$(dfx canister call template pause_query 2>&1)" "(false)"
test "business_example_query" "$(dfx canister call template business_example_query 2>&1)" "test string"

# test completed

echo ""
green "=================== TEST COMPLETED AND SUCCESSFUL ==================="
echo ""

say test successful

# sleep 10000
# read -s -n1 -p "按任意键结束..."
