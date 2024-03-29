let trace_id = 0;
function next_trace_id() {
    trace_id += 1;
    return trace_id
}
function subscribe(event, broker_id)  {
    try {
        if(!window.connections[broker_id]) {
            return
        }
        let trace_id = next_trace_id();
        var form = document.getElementById('form-subscribe-' + broker_id);
        var formData = new FormData(form);
        var formObject = {};
        formData.forEach(function(value, key){
            formObject[key] = value;
        });
        let input = document.getElementById('subscribe_topic_input_' + broker_id);
        if (input) {
            if (!formObject["topic"]) {
                input.classList.add('input-error');
                return;
            } else {
                input.classList.remove('input-error')
            }
        } else {
            return;
        }
        formObject["trace_id"] = trace_id;
        formObject["broker_id"] = broker_id;
        formObject["qos"] = get_qos(formObject["qos"]);
        remove_subcribe(broker_id, formObject["topic"]);
        window.subscribes[broker_id][formObject["topic"]] = {
            ty: formObject["payload_ty"],
            topic: formObject["topic"],id: trace_id
        };
        init_subscribe_item(trace_id, formObject["topic"], formObject["qos"], formObject["payload_ty"], broker_id, get_time());
        let rs = get_invoke()("subscribe", { datas : formObject});
        console.log(rs);
        event.target.classList.add('bg-yellow-600');
        setTimeout(() => {
            event.target.classList.remove('bg-yellow-600');
            event.target.classList.add('bg-yellow-500');
        }, 200);
    } catch(e) {
        console.error("Parsing error:", e);
    }
}

async function remove_subcribe(broker_id, topic) {
    if (window.subscribes[broker_id][topic]) {
        if (window.subscribes[broker_id][topic].id) {
            let element = document.getElementById("subscribe_" + window.subscribes[broker_id][topic].id);
            if(element) {
                element.remove();
            }
        }
    }
}

function unsubscribe(subscribe_id, topic, broker_id)  {
    let div_id = "subscribe_" + subscribe_id;
    try {
        const element = document.getElementById(div_id);
        if (element) {
            element.remove();
        }
        delete window.subscribes[broker_id].topic;
        let rs = get_invoke()("unsubscribe", { brokerId : broker_id, topic: topic});
        console.log(rs);
    } catch(e) {
        console.error("unsubscribe error:", e);
    }
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
        tableBody = document.getElementById("subs_his_modal");

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
        tableBody = document.getElementById("publish_his_modal");

        // 设置模态窗口的位置
        tableBody.style.display = 'block';
        tableBody.style.top = rect.top + 'px'; // 或者使用 rect.bottom + 'px'，取决于需要
    } catch(e) {
        console.error("Parsing error:", e);
    }
}

async function delete_publish_his(topic, qos, ty, payload, retain, broker_id) {
    try {
        var formObject = {};
        formObject["topic"] = topic;
        formObject["payload_ty"] = ty;
        formObject["qos"] = qos;
        formObject["msg"] = payload;
        formObject["retain"] = retain;
        get_invoke()("delete_publish_his", { brokerId : broker_id, his: formObject});
        let jsonObj = await get_invoke()("publish_his", { brokerId : broker_id});
        var tableBody = document.getElementById("publish_his");
        while (tableBody.firstChild) {
            tableBody.removeChild(tableBody.firstChild);
        }
        jsonObj.forEach(function(item) {
            let div = init_publish_his_item(broker_id, item["topic"], item["qos"], item["payload_ty"]
                , item["msg"], item["retain"]);
            tableBody.appendChild(div);
        });
    } catch(e) {
        console.error("unsubscribe error:", e);
    }
}

async function delete_subscribe_his(topic, qos, ty, broker_id) {
    try {
        var formObject = {};
        formObject["topic"] = topic;
        formObject["payload_ty"] = ty;
        formObject["qos"] = qos;
        get_invoke()("delete_subscribe_his", { brokerId : broker_id, his: formObject});
        let jsonObj = await get_invoke()("subscribe_his", { brokerId : broker_id});
        var tableBody = document.getElementById("subs_his");
        while (tableBody.firstChild) {
            tableBody.removeChild(tableBody.firstChild);
        }
        jsonObj.forEach(function(item) {
            let div = init_subscribe_his_item(broker_id, item["topic"], item["qos"], item["payload_ty"]);
            tableBody.appendChild(div);
        });
    } catch(e) {
        console.error("unsubscribe error:", e);
    }
}

function clear_publish(broker_id) {
    var tableBody = document.getElementById(broker_id + 'publish'); // 目标元素
    while (tableBody.firstChild) {
        tableBody.removeChild(tableBody.firstChild);
    }
}

function publish(event, broker_id)  {
    try {
        if(!window.connections[broker_id]) {
            return
        }
        var form = document.getElementById('form-publish-' + broker_id);
        var formData = new FormData(form);
        var formObject = {};
        formData.forEach(function(value, key){
            formObject[key] = value;
        });
        let topic_input = document.getElementById('publish_topic_input_' + broker_id);
        let payload_input = document.getElementById('publish_payload_input_' + broker_id);
        let check_rs = false;
        if (topic_input && payload_input) {
            if (!formObject["topic"]) {
                check_rs = true;
                topic_input.classList.add('input-error')
            } else {
                topic_input.classList.remove('input-error')
            }
            if (!formObject["msg"]) {
                check_rs = true;
                payload_input.classList.add('input-error')
            } else {
                payload_input.classList.remove('input-error')
            }
        } else {
            console.error('not found publish_topic_input_' + broker_id + ' or publish_payload_input_' + broker_id )
            return;
        }
        if (check_rs) {
            return;
        }
        let payload_ty_input = document.getElementById('publish_payload_ty_input_' + broker_id);
        if (payload_ty_input) {
            if (formObject["payload_ty"] === "Hex") {
                if (!hexStringToByteArray(formObject["msg"])) {
                    payload_ty_input.classList.add('select-error');
                    payload_input.classList.add('input-error');
                    return;
                }
            } else if (formObject["payload_ty"] === "Json") {
                if (!strToJson(formObject["msg"])) {
                    payload_ty_input.classList.add('select-error');
                    payload_input.classList.add('input-error');
                    return;
                }
            }
        } else {
            console.error('not found publish_payload_ty_input_' + broker_id )
            return;
        }
        payload_ty_input.classList.remove('select-error');
        payload_input.classList.remove('input-error');

        formObject["trace_id"] = next_trace_id();
        formObject["broker_id"] = broker_id;
        formObject["qos"] = get_qos(formObject["qos"]);
        formObject["retain"] = check_to_bool(formObject["retain"]);
        var json = JSON.stringify(formObject);
        console.log("publish:" + json);
        init_publish_item(formObject["trace_id"], formObject["topic"], formObject["msg"]
            , formObject["qos"], formObject["payload_ty"], broker_id, get_time(), formObject["retain"])
        let rs = get_invoke()("publish", { datas : formObject});
        console.log(rs);

        event.target.classList.add('bg-yellow-600');
        setTimeout(() => {
            event.target.classList.remove('bg-yellow-600');
            event.target.classList.add('bg-yellow-500');
        }, 200);
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
function strToJson(hexString) {
    try {
        let byte = JSON.parse(hexString);
        console.debug("JSON:" + byte);
        return true;
    } catch (_) {
        return false;
    }
}
function hexStringToByteArray(hexString) {
    hexString = hexString.replaceAll(" ", "");
    if (hexString.length % 2 > 0) {
        return false;
    }
    try {
        for (let i = 0; i < hexString.length; i += 2) {
            let hexByte = hexString.substring(i, i + 2);
            let byte = parseInt(hexByte, 16);
            if (isNaN(byte)) {
                return false;
            }
            console.debug("byte:" + byte);
        }
        return true;
    } catch (_) {
        return false;
    }
}