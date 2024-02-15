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
    } else {
        return 2
    }
}