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
    let a_class = "bg-white inline-block py-2 px-1 text-gray-500 hover:text-teal-800 font-semibold flex";
    let i_class = "layui-icon layui-icon-close py-2 px-1";
    let li_class = "mr-1 shadow rounded-md justify-center";
    let span_class = "h-3 w-3 bg-green-400 rounded-full mr-2 py-2 px-1";
    // <a className="bg-white inline-block py-2 px-1 text-gray-500 hover:text-teal-800 font-semibold flex" href="#second">
    //     <span className="h-3 w-3 bg-green-400 rounded-full mr-2 py-2 px-1 "></span>
    //     Tab 2</a>
    var span = document.createElement('span');
    span.className = span_class;
    var name = document.createTextNode(name);
    var name_a = document.createElement('a');
    name_a.className = a_class;
    name_a.appendChild(span);
    name_a.appendChild(name);

    // <a className="bg-white inline-block py-2 px-1 text-gray-500 hover:text-teal-800 font-semibold flex" href="#second">
    //     <i className="layui-icon layui-icon-close py-2 px-1 "></i></a>
    var i = document.createElement('i');
    i.className = i_class;
    var i_a = document.createElement('a');
    i_a.className = a_class;
    i_a.appendChild(i);

    // <li class="mr-1 shadow rounded-md justify-center">
    //     <div class="flex px-4">
    //         <a></a>
    //         <a></a>
    //     </div>
    // </li>
    var div = document.createElement('div');
    div.className = "flex px-4";
    div.appendChild(name_a);
    div.appendChild(i_a);
    var li = document.createElement('li');
    li.className = li_class;
    li.appendChild(div);
    return li;
}