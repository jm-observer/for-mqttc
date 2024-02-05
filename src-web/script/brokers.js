function get_invoke() {
    if (isTauriEnvironment()) {
        // Tauri特有的API
        return window.__TAURI__.tauri.invoke
    } else {
        return function (method, data) {
            console.log(method, data);
            return method
        }
    }
}

function isTauriEnvironment() {
    return typeof window.__TAURI__ !== 'undefined';
}


//
// let greetInputEl;
// let greetMsgEl;
async function broker_list() {
    try {
        let rs = await get_invoke()("broker_list", { page : {start: 0, size: 10 }});
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

async function connect_to_broker(id, name) {
    console.log("connect_to_broker" + id);
    const tableBody = document.getElementById("tabs");
    const tab = init_tab(id, name);
    tableBody.appendChild(tab);

    fetch('connecting_template.html')
        .then(response => {
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            return response.text();
        })
        .then(htmlString => {
            var parser = new DOMParser();
            let content = htmlString.replaceAll("__id__", id);
            var doc = parser.parseFromString(content, 'text/html');
            return doc.body.children[0]; // 或者 doc.documentElement，视情况而定
        })
        .then(htmlElement => {
            var targetElement = document.getElementById('tabs-content'); // 目标元素
            targetElement.appendChild(htmlElement);
        })
        .catch(error => {
            console.error('There has been a problem with your fetch operation:', error);
        });

    await get_invoke()("connect_to_broker", { id : id});
}
function delete_broker(id) {
    console.log("delete_broker" + id);
}

function edit_broker(id) {
    console.log("edit_broker" + id);
}

function display_tab(tab_id) {
    console.log("display_tab" + tab_id);
    let parentElement = document.getElementById('tabs');
    for (let i = 0; i < parentElement.children.length; i++) {
        let tab = parentElement.children[i];
        if(tab.id.endsWith(tab_id)) {
            tab.classList.remove('text-gray-500');
            tab.classList.add('text-teal-500');
        } else {
            tab.classList.remove('text-teal-500');
            tab.classList.add('text-gray-500');
        }
    }

    parentElement = document.getElementById('tabs-content');
    for (let i = 0; i < parentElement.children.length; i++) {
        let tab = parentElement.children[i];
        if(tab.id.endsWith(tab_id)) {
            tab.style.display = 'block';
        } else {
            tab.style.display = 'none';
        }
    }
}


async function close_tab(broker_id) {
    console.log("close_tab: " + broker_id);
    let tab_id = 'tab-' + broker_id;
    var element = document.getElementById(tab_id);
    if (element) {
        element.parentNode.removeChild(element);
    }
    let tab_content_id = 'tab-content-' + broker_id;
    var element = document.getElementById(tab_content_id);
    if (element) {
        element.parentNode.removeChild(element);
    }
    display_tab("brokers");
    await get_invoke()("disconnect", { id : broker_id});
}

//
window.addEventListener("DOMContentLoaded", () => {
    display_tab("brokers");
    broker_list();
});


