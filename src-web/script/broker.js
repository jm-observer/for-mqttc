

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

    document.getElementById('new').addEventListener('click', function(event) {
        var modal = document.getElementById('modal');
        var triggerButton = document.getElementById('tabs-content');
        // 获取触发按钮的位置
        var rect = triggerButton.getBoundingClientRect();

        // 设置模态窗口的位置
        modal.style.display = 'block';
        modal.style.top = rect.top + 'px'; // 或者使用 rect.bottom + 'px'，取决于需要
        event.stopPropagation();
    });

    document.getElementById('main').addEventListener('click', function(event) {
        var modal = document.getElementById('modal');
        if (event.target != modal && modal.style.display == 'block') {
            modal.style.display = 'none';
        }
    });

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
        document.getElementById('self_signed_ca_div').add('hidden');
    });
    document.getElementById('tls-ca-label').addEventListener('click', function(event) {
        document.getElementById('tls-ca').checked = true;
        document.getElementById('self_signed_ca_div').add('hidden');
    });
    document.getElementById('tls-insecurity-label').addEventListener('click', function(event) {
        document.getElementById('tls-insecurity').checked = true;
        document.getElementById('self_signed_ca_div').add('hidden');
    });
    document.getElementById('tls-self-signed-label').addEventListener('click', function(event) {
        document.getElementById('tls-self-signed').checked = true;
        document.getElementById('self_signed_ca_div').remove('hidden');
    });
}

function check_values() {
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
            document.getElementById('tls-self-signed').classList.add('file-input-error')
        } else {
            document.getElementById('tls-self-signed').classList.remove('file-input-error')
        }
    }

    try {
        const _ = JSON.parse(formObject["params"]);
        document.getElementById('params').classList.remove('textarea-error')
    } catch (error) {
        result = false;
        document.getElementById('params').classList.add('textarea-error')
    }

    if (result) {
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

