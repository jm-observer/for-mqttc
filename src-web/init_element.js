 function init_buttons(newRow, id, name) {
    var className = "bg-yellow-500 hover:bg-yellow-700 font-bold py-1 px-2 rounded focus:outline-none focus:shadow-outline mr-1";
    var cell = newRow.insertCell();
    var button = document.createElement('button');
    button.innerHTML = '连接'; // 设置按钮文本
    button.className = className; // 设置类名
    button.addEventListener('click', function () {
        connect_to_broker(id, name);
    });
    cell.appendChild(button);

    var button = document.createElement('button');
    button.innerHTML = '编辑'; // 设置按钮文本
    button.className = className; // 设置类名
    button.addEventListener('click', function () {
        edit_broker(id);
    });
    cell.appendChild(button);

    var button = document.createElement('button');
    button.innerHTML = '删除'; // 设置按钮文本
    button.className = className; // 设置类名
    button.addEventListener('click', function () {
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

 function init_tab(id, name) {
    let template = "<li id='tab-#id#' class='mr-1 shadow rounded-md justify-center'>\n" +
        "                            <div class='flex px-4'>\n" +
        "                                <a onclick='display_tab(\"#id#\")' class='bg-white inline-block py-2 px-1 text-gray-500 hover:text-teal-800 font-semibold flex' href='#'>\n" +
        "                                    <span class='h-3 w-3 bg-green-400 rounded-full mr-2 py-2 px-1 '></span>\n" +
        "                                    #name#</a>\n" +
        "                                <i class='layui-icon layui-icon-close py-2 px-1 '></i>\n" +
        "                            </div>\n" +
        "                        </li>";

     const htmlString = template.replaceAll("#id#", id).replaceAll("#name#", name);

     var tempDiv = document.createElement('div');
     tempDiv.innerHTML = htmlString;
     return tempDiv.children[0];
}