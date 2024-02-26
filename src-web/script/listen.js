const listen = window.__TAURI__.event.listen;

listen('ClientConnectAckSuccess', (event) => {
    console.log("ClientConnectAckSuccess:" + event.payload.broker_id);
    let status = document.getElementById("status-" + event.payload.broker_id);
    status.classList.remove("bg-gray-400");
    status.classList.add("bg-green-400");
});

listen('ClientSubAck', (event) => {
    console.log("ClientSubAck:" + event.payload.broker_id + " " + event.payload.trace_id);
    let status = document.getElementById("subcribe-status-" + event.payload.trace_id);
    status.classList.remove("bg-gray-400");
    status.classList.add("bg-green-400");
});

listen('ClientPubAck', (event) => {
    console.log("ClientPubAck:" + event.payload.broker_id + " " + event.payload.trace_id);
    let status = document.getElementById("publish-status-" + event.payload.trace_id);
    status.classList.remove("bg-gray-400");
    status.classList.add("bg-green-400");
});


listen('ClientReceivePublic', (event) => {
    console.log("ClientReceivePublic:" + event.payload.broker_id
        + " " + event.payload.topic
        + " " + event.payload.qos
        + " " + event.payload.payload);
    let topic = event.payload.topic;
    let payload_ty = "Text";
    let broker_id = event.payload.broker_id;
    for (var key in window.subscribes[broker_id]) {
        var topic_payload_ty = window.subscribes[broker_id][key].ty;
        if (key === "#") {
            payload_ty = topic_payload_ty;
            break;
        } else if (key.endsWith("/#")) {
            // todo key.substring(0, key.length - 2) end /
            if (topic.startsWith(key.substring(0, key.length - 2))) {
                payload_ty = topic_payload_ty;
                break;
            }
        } else if (key === "+") {
            if (!topic.includes('/')) {
                payload_ty = topic_payload_ty;
                break;
            }
        } else if (key.endsWith("/+")) {
            let front_key = key.substring(0, key.length - 1);
            if (topic.startsWith(front_key)) {
                let sub_topic = topic.substring(front_key.length, topic.length);
                if (!sub_topic.includes('/')) {
                    payload_ty = topic_payload_ty;
                    break;
                }
            }
        } else if (key === topic){
            payload_ty = topic_payload_ty;
            break;
        }
    }
    var byteStream = new Uint8Array(event.payload.payload);
    let payload = parse_payload(payload_ty, byteStream);
    init_receive_publish_item(next_trace_id(), event.payload.topic, payload, event.payload.qos, payload_ty, event.payload.broker_id, get_time(), byteStream)
});

listen('ClientDisconnect', (event) => {
    console.log("ClientDisconnect:" + event.payload.broker_id);
});

function byteArrayToHex(byteArray) {
    const hexes = [];
    for (let i = 0; i < byteArray.length; i++) {
        const hex = (byteArray[i] & 0xFF).toString(16);
        hexes.push(hex.length === 1 ? '0' + hex : hex);
    }
    return hexes.join(' ')
}

function parse_payload(payload_ty, byteStream) {
    let utf8String;
    if (payload_ty === "Hex") {
        utf8String = byteArrayToHex(byteStream);
    } else {
        var decoder = new TextDecoder('utf-8');
        utf8String = decoder.decode(byteStream);
        if (payload_ty === "Json") {
            try {
                let obj = JSON.parse(utf8String);
                utf8String = JSON.stringify(obj, null, 4);
            } catch (e) {
                console.error('Json fail :', e);
            }
        }
    }
    return utf8String;
}

