const listen = window.__TAURI__.event.listen;

listen('ClientConnectAckSuccess', (event) => {
    console.log("ClientConnectAckSuccess:" + event.payload.broker_id);
    let status = document.getElementById("status-" + event.payload.broker_id);
    status.classList.remove("bg-gray-400");
    status.classList.add("bg-green-400");
});

listen('ClientSubAck', (event) => {
    console.log("ClientSubAck:" + event.payload.broker_id + " " + event.payload.trace_id);
    // let status = document.getElementById("status-" + event.payload.broker_id);
    // status.classList.remove("bg-gray-400");
    // status.classList.add("bg-green-400");
});

listen('ClientPubAck', (event) => {
    console.log("ClientPubAck:" + event.payload.broker_id + " " + event.payload.trace_id);
    // let status = document.getElementById("status-" + event.payload.broker_id);
    // status.classList.remove("bg-gray-400");
    // status.classList.add("bg-green-400");
});


listen('ClientReceivePublic', (event) => {
    console.log("ClientReceivePublic:" + event.payload.broker_id
        + " " + event.payload.topic
        + " " + event.payload.qos
        + " " + event.payload.payload);
    let topic = event.payload.topic;
    let payload_ty = "Text";
    for (var key in window.subscribes) {
        if (key === "#") {
            payload_ty = window.subscribes[key];
            break;
        } else if (key.endsWith("/#")) {
            // todo key.substring(0, key.length - 2) end /
            if (topic.startsWith(key.substring(0, key.length - 2))) {
                payload_ty = window.subscribes[key];
                break;
            }
        } else if (key === "+") {
            if (!topic.contains('/')) {
                payload_ty = window.subscribes[key];
                break;
            }
        } else if (key.endsWith("/+")) {
            let sub_topic = key.substring(0, key.length - 2);
            if (topic.startsWith(sub_topic)) {
                if (!topic.substring(sub_topic.length, topic.length).contains('/')) {
                    payload_ty = window.subscribes[key];
                    break;
                }
            }
        } else if (key === topic){
            payload_ty = window.subscribes[key];
            break;
        }
    }
    if (payload_ty == "Hex") {
        var byteStream = new Uint8Array(event.payload.payload);
        const utf8String = byteArrayToHex(byteStream);
        init_receive_publish_item(next_trace_id(), event.payload.topic, utf8String, event.payload.qos, payload_ty, event.payload.broker_id, get_time())
    } else {
        var byteStream = new Uint8Array(event.payload.payload);
        var decoder = new TextDecoder('utf-8');
        var utf8String = decoder.decode(byteStream);
        if (payload_ty == "Json") {
            try {
                let obj = JSON.parse(utf8String);
                utf8String = JSON.stringify(obj, null, 4);
            } catch (e) {
                console.error('Json fail :', e);
            }
        }
        init_receive_publish_item(next_trace_id(), event.payload.topic, utf8String, event.payload.qos, payload_ty, event.payload.broker_id, get_time())
    }
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



