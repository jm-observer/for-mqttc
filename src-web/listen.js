const listen = window.__TAURI__.event.listen;

// listen to the `click` event and get a function to remove the event listener
// there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
listen('ClientConnectAckSuccess', (event) => {
    console.log("ClientConnectAckSuccess:" + event.payload.broker_id);
    let status = document.getElementById("status-" + event.payload.broker_id);
    status.classList.remove("bg-gray-400");
    status.classList.add("bg-green-400");
});

listen('ClientSubAck', (event) => {
    console.log("ClientSubAck" + event.payload.broker_id + " " + event.payload.trace_id);
    // let status = document.getElementById("status-" + event.payload.broker_id);
    // status.classList.remove("bg-gray-400");
    // status.classList.add("bg-green-400");
});






