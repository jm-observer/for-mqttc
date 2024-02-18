let trace_id = 0;

function next_trace_id() {
    trace_id += 1;
    return trace_id
}
function subscribe(broker_id)  {
    try {
        let trace_id = next_trace_id();
        var form = document.getElementById('form-subscribe-' + broker_id);
        var formData = new FormData(form);
        var formObject = {};
        formData.forEach(function(value, key){
            formObject[key] = value;
        });
        formObject["trace_id"] = trace_id;
        formObject["broker_id"] = broker_id;
        formObject["qos"] = get_qos(formObject["qos"]);

        init_subscribe_item(trace_id, formObject["topic"], formObject["qos"], formObject["payload_ty"], broker_id, get_time());
        let rs = get_invoke()("subscribe", { datas : formObject});
        console.log(rs);
    } catch(e) {
        console.error("Parsing error:", e);
    }
}

function unsubscribe(subscribe_id)  {
    console.log("unsubscribe:" + subscribe_id);
}

async function display_subscribe_his(event, broker_id){
    try {
        let jsonObj = await get_invoke()("subscribe_his", { brokerId : broker_id});
        // var jsonObj = JSON.parse(rs);
        var tableBody = document.getElementById("subs_his");
        while (tableBody.firstChild) {
            tableBody.removeChild(tableBody.firstChild);
        }
        jsonObj.forEach(function(item) {
            let div = init_subscribe_his_item(broker_id, item["topic"], item["qos"], item["payload_ty"]);
            tableBody.appendChild(div);
        });
        var triggerButton = document.getElementById('tabs-content');
        // 获取触发按钮的位置
        var rect = triggerButton.getBoundingClientRect();
        var tableBody = document.getElementById("subs_his_modal");

        // 设置模态窗口的位置
        tableBody.style.display = 'block';
        tableBody.style.top = rect.top + 'px'; // 或者使用 rect.bottom + 'px'，取决于需要
        tableBody.style.left = event.target.getBoundingClientRect().left + 'px';
    } catch(e) {
        console.error("Parsing error:", e);
    }
}


async function display_publish_his(event, broker_id){
    try {
        let jsonObj = await get_invoke()("publish_his", { brokerId : broker_id});
        // var jsonObj = JSON.parse(rs);
        var tableBody = document.getElementById("publish_his");
        while (tableBody.firstChild) {
            tableBody.removeChild(tableBody.firstChild);
        }
        jsonObj.forEach(function(item) {
            let div = init_publish_his_item(broker_id, item["topic"], item["qos"], item["payload_ty"]
                , item["msg"], item["retain"]);
            tableBody.appendChild(div);
        });
        var triggerButton = document.getElementById('tabs-content');
        // 获取触发按钮的位置
        var rect = triggerButton.getBoundingClientRect();
        var tableBody = document.getElementById("publish_his_modal");

        // 设置模态窗口的位置
        tableBody.style.display = 'block';
        tableBody.style.top = rect.top + 'px'; // 或者使用 rect.bottom + 'px'，取决于需要
    } catch(e) {
        console.error("Parsing error:", e);
    }
}

function publish(broker_id)  {
    try {
        var form = document.getElementById('form-publish-' + broker_id);
        var formData = new FormData(form);
        var formObject = {};
        formData.forEach(function(value, key){
            formObject[key] = value;
        });
        formObject["trace_id"] = next_trace_id();
        formObject["broker_id"] = broker_id;
        formObject["qos"] = get_qos(formObject["qos"]);
        formObject["retain"] = check_to_bool(formObject["retain"]);
        var json = JSON.stringify(formObject);
        console.log("publish:" + json);

        init_publish_item(formObject["trace_id"], formObject["topic"], formObject["msg"]
            , formObject["qos"], formObject["payload_ty"], broker_id, get_time())
        let rs = get_invoke()("publish", { datas : formObject});
        console.log(rs);
    } catch(e) {
        console.error("Parsing error:", e);
    }
}
function get_qos(qos) {
    if (qos === "0") {
        return 0
    } else if (qos === "1") {
        return 1
    } else if (qos === "2"){
        return 2
    } else {
        return qos
    }
}