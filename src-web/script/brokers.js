let brokers;


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


async function loading() {
    await get_invoke()("loading");
}

function isTauriEnvironment() {
    return typeof window.__TAURI__ !== 'undefined';
}


async function broker_list() {
    try {
        let rs = await get_invoke()("broker_list");
        console.log(rs);
        var jsonObj = JSON.parse(rs);
        var tableBody = document.getElementById("brokers").getElementsByTagName('tbody')[0];

        while (tableBody.firstChild) {
            tableBody.removeChild(tableBody.firstChild);
        }

        brokers = jsonObj.brokers;

        brokers.forEach(function(item) {
            var newRow = tableBody.insertRow();
            init_version_cell(newRow, item["version"]);
            init_name_cell(newRow, item["name"]);
            init_common_cell(newRow, item["tls"]);
            init_common_cell(newRow, item["addr"] + ":" + item["port"]);
            init_buttons(newRow, item["id"], item["name"]);
            newRow.addEventListener('dblclick', function (event) {
                event.stopPropagation();
                connect_to_broker(item["id"], item["name"]);
            });
        });
    } catch (e) {
        console.error("Parsing error:", e);
    }
}

async function connect_to_broker(id, name) {
    console.log("connect_to_broker" + id);
    await close_tab(id);
    const tableBody = document.getElementById("tabs");
    const tab = document.getElementById("tab-" + id);
    if(!tab) {
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
        window.subscribes[id] = {};
        await get_invoke()("connect_to_broker", { id : id});
    }
    display_tab(id);
}
async function delete_broker(id) {
    console.log("delete_broker " + id);
    await get_invoke()("delete_broker", { id : id});
    await close_tab(id);
    await broker_list();
}

function edit_broker(id) {
    for (const item of brokers) {
        if(item.id === id) {
            init_broker_value(item.id, item.name, item.client_id, item.addr, item.port, item.auto_connect
                , item.credential
                , item.user_name
                , item.password
                , item.version
                , item.tls
                , item.self_signed_ca, item.params);
            display_broker_info()
        }
    }
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
    element = document.getElementById(tab_content_id);
    if (element) {
        element.parentNode.removeChild(element);
    }
    display_tab("brokers");
    window.subscribes[broker_id] = {};
    await get_invoke()("disconnect", { id : broker_id});
}

//
window.addEventListener("DOMContentLoaded", () => {
    display_tab("brokers");
    broker_list();
});


