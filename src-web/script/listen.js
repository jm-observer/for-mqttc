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

    var byteStream = new Uint8Array(event.payload.payload);
    console.log(byteStream);

    var decoder = new TextDecoder('utf-8');
    var utf8String = decoder.decode(byteStream);

    console.log(utf8String); // 输出: "Hello"
    // let status = document.getElementById("status-" + event.payload.broker_id);
    // status.classList.remove("bg-gray-400");
    // status.classList.add("bg-green-400");
});





