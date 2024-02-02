// import {init_buttons, init_common_cell, init_name_cell, init_tab, init_version_cell} from "./init_element.js";

// const { invoke } = window.__TAURI__.tauri;
//
// let greetInputEl;
// let greetMsgEl;
async function broker_list() {
    try {
        let rs = await invoke("broker_list", { page : {start: 0, size: 10 }});
        console.log(rs);
        var jsonObj = JSON.parse(rs);
        var tableBody = document.getElementById("brokers").getElementsByTagName('tbody')[0];
        jsonObj.brokers.forEach(function(item) {
            var newRow = tableBody.insertRow();
            init_version_cell(newRow, item["protocol"]);
            init_name_cell(newRow, item["name"]);
            init_common_cell(newRow, item["tls"]);
            init_common_cell(newRow, item["addr"] + ":" + item["port"]);
            init_buttons(newRow, item["id"], item["name"])
        });
    } catch (e) {
        console.error("Parsing error:", e);
    }

}

function connect_to_broker(id, name) {
    console.log("connect_to_broker" + id);
    const tableBody = document.getElementById("tabs");
    tableBody.appendChild(init_tab(id, name));

}
function delete_broker(id) {
    console.log("delete_broker" + id);
}

function edit_broker(id) {
    console.log("edit_broker" + id);
}

//
window.addEventListener("DOMContentLoaded", () => {
    broker_list();
});