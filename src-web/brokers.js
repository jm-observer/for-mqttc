const { invoke } = window.__TAURI__.tauri;
//
// let greetInputEl;
// let greetMsgEl;
async function broker_list() {
    let rs = await invoke("broker_list", { page : {start: 0, size: 10 }});
    console.log(rs);
    try {
        var jsonObj = JSON.parse(rs);
        var tableBody = document.getElementById("brokers").getElementsByTagName('tbody')[0];
        jsonObj.brokers.forEach(function(item) {
            var newRow = tableBody.insertRow();
            init_version_cell(newRow, item["protocol"]);
            init_version_cell(newRow, item["name"]);
            init_common_cell(newRow, item["tls"]);
            init_common_cell(newRow, item["addr"] + ":" + item["port"]);
            init_buttons(newRow, item["id"])
        });
    } catch (e) {
        console.error("Parsing error:", e);
    }

}

function connect_to_broker(id) {
    console.log("connect_to_broker" + id);
}
function delete_broker(id) {
    console.log("delete_broker" + id);
}

function edit_broker(id) {
    console.log("edit_broker" + id);
}

function init_buttons(newRow, id) {
    var className = "bg-yellow-500 hover:bg-yellow-700 font-bold py-1 px-2 rounded focus:outline-none focus:shadow-outline mr-1";
    var cell = newRow.insertCell();
    var button = document.createElement('button');
    button.innerHTML = '连接'; // 设置按钮文本
    button.className = className; // 设置类名
    button.addEventListener('click', function() {
        connect_to_broker(id);
    });
    cell.appendChild(button);

    var button = document.createElement('button');
    button.innerHTML = '编辑'; // 设置按钮文本
    button.className = className; // 设置类名
    button.addEventListener('click', function() {
        edit_broker(id);
    });
    cell.appendChild(button);

    var button = document.createElement('button');
    button.innerHTML = '删除'; // 设置按钮文本
    button.className = className; // 设置类名
    button.addEventListener('click', function() {
        delete_broker(id);
    });
    cell.appendChild(button);
}
function init_version_cell(newRow, text) {
    var newCell = newRow.insertCell();
    newCell.classList.add("w-12", "text-center");
    var newText = document.createTextNode(text);
    newCell.appendChild(newText);
}
function init_name_cell(newRow, text) {
    var newCell = newRow.insertCell();
    newCell.classList.add("w-36", "text-center");
    var newText = document.createTextNode(text);
    newCell.appendChild(newText);
}
function init_common_cell(newRow, text) {
    var newCell = newRow.insertCell();
    var newText = document.createTextNode(text);
    newCell.appendChild(newText);
}
//
window.addEventListener("DOMContentLoaded", () => {
    broker_list();
});