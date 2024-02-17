

async function init_broker_model() {
    document.getElementById('credential').addEventListener('change', function() {
        var user_name = document.getElementById('user_name_div');
        var password = document.getElementById('password_div');
        if (this.checked) {
            user_name.classList.remove('hidden'); // 显示 input
            password.classList.remove('hidden');
        } else {
            user_name.classList.add('hidden'); // 显示 input
            password.classList.add('hidden');
        }
    });

    document.querySelectorAll('input[name="tls"]').forEach((radio) => {
        radio.addEventListener('change', function() {
            var inputElement = document.getElementById('self_signed_ca_div');
            if (document.getElementById('tls-self-signed').checked) {
                inputElement.classList.remove('hidden'); // 显示 input
            } else  {
                inputElement.classList.add('hidden'); // 隐藏 input
            }
        });
    });

    const new_button = document.getElementById('new');
    if (new_button) {
        new_button.addEventListener('click', function(event) {
            event.stopPropagation();
            display_broker_info();
            init_new_broker();
        });
    }
    const main = document.getElementById('main');
    if (main) {
        main.addEventListener('click', function(event) {
            var modal = document.getElementById('modal');
            if (event.target != modal && modal.style.display == 'block') {
                modal.style.display = 'none';
            }
        });
    }


    document.getElementById('self_signed_ca').addEventListener('click', function(event) {
        select_file()
    });

    document.getElementById('version-v3-label').addEventListener('click', function(event) {
        document.getElementById('version-v3').checked = true;
    });
    document.getElementById('version-v5-label').addEventListener('click', function(event) {
        document.getElementById('version-v5').checked = true;
    });

    document.getElementById('tls-none-label').addEventListener('click', function(event) {
        document.getElementById('tls-none').checked = true;
        document.getElementById('self_signed_ca_div').classList.add('hidden');
    });
    document.getElementById('tls-ca-label').addEventListener('click', function(event) {
        document.getElementById('tls-ca').checked = true;
        document.getElementById('self_signed_ca_div').classList.add('hidden');
    });
    document.getElementById('tls-insecurity-label').addEventListener('click', function(event) {
        document.getElementById('tls-insecurity').checked = true;
        document.getElementById('self_signed_ca_div').classList.add('hidden');
    });
    document.getElementById('tls-self-signed-label').addEventListener('click', function(event) {
        document.getElementById('tls-self-signed').checked = true;
        document.getElementById('self_signed_ca_div').classList.remove('hidden');
    });
    document.getElementById('self_signed_ca_div').classList.add('hidden');
}

async function check_values() {
    var form = document.getElementById('broker');
    var formData = new FormData(form);
    var formObject = {};
    formData.forEach(function(value, key){
        formObject[key] = value;
    });
    let result = true;
    if (!formObject["name"] ) {
        result = false;
        document.getElementById('name').classList.add('input-error')
    } else {
        document.getElementById('name').classList.remove('input-error')
    }
    if (!formObject["client_id"] ) {
        result = false;
        document.getElementById('client_id').classList.add('input-error')
    } else {
        document.getElementById('client_id').classList.remove('input-error')
    }

    if (!formObject["addr"] ) {
        result = false;
        document.getElementById('addr').classList.add('input-error')
    } else {
        document.getElementById('addr').classList.remove('input-error')
    }

    if (!formObject["port"] ) {
        result = false;
        document.getElementById('port').classList.add('input-error')
    } else {
        document.getElementById('port').classList.remove('input-error')
    }
    formObject["auto_connect"] = check_to_bool(formObject["auto_connect"])
    formObject["credential"] = check_to_bool(formObject["credential"])
    if (formObject["credential"]) {
        if (!formObject["user_name"] ) {
            result = false;
            document.getElementById('user_name').classList.add('input-error')
        } else {
            document.getElementById('user_name').classList.remove('input-error')
        }
        if (!formObject["password"] ) {
            result = false;
            document.getElementById('password').classList.add('input-error')
        } else {
            document.getElementById('password').classList.remove('input-error')
        }
    }

    if (formObject["tls"] === "self_signed") {
        if (formObject["self_signed_ca"] === "" ) {
            result = false;
            document.getElementById('self_signed_ca').classList.add('file-input-error')
        } else {
            document.getElementById('self_signed_ca').classList.remove('file-input-error')
        }
    }
    formObject["port"] = Number(formObject["port"]);
    formObject["id"] = Number(formObject["id"]);
    if (!formObject["port"]) {
        result = false;
        document.getElementById('port').classList.add('input-error')
    } else {
        document.getElementById('port').classList.remove('input-error')
    }
    try {
        const _ = JSON.parse(formObject["params"]);
        document.getElementById('params').classList.remove('textarea-error')
    } catch (error) {
        result = false;
        document.getElementById('params').classList.add('textarea-error')
    }

    if (result) {
        try {
            let rs = await window.__TAURI__.tauri.invoke("update_or_new_broker", { broker : formObject});
            broker_list();
        } catch (e) {
            console.error("Parsing error:", e);
        }
    }
}

async function select_file() {
    const open = window.__TAURI__.dialog.open;
    const selected = await open({
        multiple: false,
        directory: false,
    });
    if (Array.isArray(selected)) {
        // user selected multiple files
    } else if (selected === null) {
        // user cancelled the selection
    } else {
        document.getElementById('self_signed_ca').value = selected;
    }
}

async function display_broker_info() {
    var modal = document.getElementById('modal');
    var triggerButton = document.getElementById('tabs-content');
    // 获取触发按钮的位置
    var rect = triggerButton.getBoundingClientRect();
    // 设置模态窗口的位置
    modal.style.display = 'block';
    modal.style.top = rect.top + 'px'; // 或者使用 rect.bottom + 'px'，取决于需要
}

async function init_new_broker() {
    if (document.getElementById('id').value === '0') {
        return
    }
    const params_obj = {"keep_alive": 60,
        "clean_session": true,
        "max_incoming_packet_size": 10240,
        "max_outgoing_packet_size": 10240,
        "inflight": 100,
        "conn_timeout": 5
    };
    let params = JSON.stringify(params_obj);
    init_broker_value(0, '', '', '',1883, true, false, '', '', 'v4', 'none', '',  params)
}

async function init_broker_value(id, name, client_id, addr, port, auto_connect, credential, user_name, password, version, tls, self_signed_ca, params) {
    document.getElementById('id').value = id;
    document.getElementById('name').value = name;
    document.getElementById('client_id').value = client_id;
    document.getElementById('addr').value = addr;
    document.getElementById('port').value = port;

    document.getElementById('auto_connect').checked = auto_connect;
    document.getElementById('credential').checked = credential;
    document.getElementById('user_name').value = user_name;
    document.getElementById('password').value = password;

    document.getElementById('params').value = params;

    if (version === "v4") {
        document.getElementById('version-v3').checked = true;
    } else if (version === "v5") {
        document.getElementById('version-v5').checked = true;
    }

    if (tls === "none") {
        document.getElementById('tls-none').checked = true;
        document.getElementById('self_signed_ca').value = '';
        document.getElementById('self_signed_ca_div').classList.add('hidden');
    } else if (tls === "ca") {
        document.getElementById('tls-ca').checked = true;
        document.getElementById('self_signed_ca').value = '';
        document.getElementById('self_signed_ca_div').classList.add('hidden');
    } else if (tls === "insecurity") {
        document.getElementById('tls-insecurity').checked = true;
        document.getElementById('self_signed_ca').value = '';
        document.getElementById('self_signed_ca_div').classList.add('hidden');
    } else if (tls === "self_signed") {
        document.getElementById('tls-self-signed').checked = true;
        document.getElementById('self_signed_ca').value = self_signed_ca;
        document.getElementById('self_signed_ca_div').classList.remove('hidden');
    }
}