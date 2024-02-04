const listen = window.__TAURI__.event.listen;

// listen to the `click` event and get a function to remove the event listener
// there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
let _ = listen('ClientConnectAckSuccess', (event) => {
    console.log("ClientConnectAckSuccess:" + event.payload.broker_id);
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
})




