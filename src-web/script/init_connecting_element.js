 async function init_subscribe_item(id, topic, qos, ty, connect_id, time) {
    let template = "        <div class=\"flex items-center pb-1 mb-2\">\n" +
        "            <span id=\"subcribe-status-__id__\" class=\"h-3 w-3 bg-gray-400 rounded-full mr-2\"></span>\n" +
        "            <span id=\"copy-subcribe-topic-__id__\" class=\"flex-grow text-gray-800\">__topic__</span>\n" +
        "        <a onclick='unsubscribe(__id__, \"__topic__\", __connect_id__)'><i class=\"ml-auto layui-icon layui-icon-close py-2 px-1 \"></i></a>\n" +
        "        </div>\n" +
        "        <span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full\">QoS __qos__</span>\n" +
        "        <span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full\">__ty__</span>\n" +
     "        <span class=\"px-2 py-1 ml-auto\">__time__</span>\n"
     ;

     const htmlString = template.replaceAll("__id__", id).replaceAll("__topic__", topic)
         .replaceAll("__qos__", qos).replaceAll("__ty__", ty).replaceAll("__time__", time)
         .replaceAll("__connect_id__", connect_id);

     var tempDiv = document.createElement('div');
     tempDiv.id = "subscribe_" + id;
     tempDiv.innerHTML = htmlString;
     tempDiv.className = "items-center justify-between p-2 bg-white rounded-lg shadow-md mb-4";
     tempDiv.addEventListener('dblclick', function () {
         unsubscribe(id, topic, connect_id);
     });

     var targetElement = document.getElementById(connect_id + 'subs'); // 目标元素
     targetElement.appendChild(tempDiv);
     targetElement.scrollTop = targetElement.scrollHeight;

     document.getElementById('copy-subcribe-topic-' + id).addEventListener('contextmenu', function(event) {
         event.preventDefault();
         navigator.clipboard.writeText(topic).then(function() {
             console.log('copy:' + topic);
         }).catch(function(error) {
             console.error('copy fail :', error);
         });
     });
}

 async function init_publish_item(id, topic, payload, qos, ty, connect_id, time, retain) {
     let template = "        <div class=\"flex items-center pb-1  \">\n" +
         "            <span id=\"publish-status-__id__\" class=\"h-3 w-3 bg-gray-400 rounded-full mr-2\"></span>\n" +
         "            <span id=\"copy-publish-topic-__id__\" class=\"flex-grow text-gray-800\">__topic__</span>\n" +
         "        </div>\n" +
         "        <div id=\"copy-publish-payload-__id__\" class=\"mb-2 px-2 py-1 flex-grow rounded-lg bg-green-200 text-gray-800\">" +
         "          <p class='clamp-2 break-words'>__payload__</p></div>\n" +
         "        <div class=\"flex justify-end\"><span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full mr-2\">QoS __qos__</span>\n" +
         "        <span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full mr-2\">__ty__</span>__retain__\n" +
         "        <span class=\"px-2 py-1 ml-auto\">__time__</span></div>\n"
     ;

     let retain_str = "";
     if (retain) {
         retain_str = "<span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full\">retain</span>";
     }

     const htmlString = template.replaceAll("__id__", id).replaceAll("__topic__", topic)
         .replaceAll("__payload__", payload)
         .replaceAll("__qos__", qos).replaceAll("__ty__", ty).replaceAll("__time__", time).replaceAll("__retain__", retain_str);

     var tempDiv = document.createElement('div');
     tempDiv.innerHTML = htmlString;
     tempDiv.className = "items-center justify-between p-2 bg-white rounded-lg shadow-md mb-2 max-w-72 ml-auto";
     // tempDiv.addEventListener('dblclick', function () {
     //     unsubscribe(id);
     // });

     var targetElement = document.getElementById(connect_id + 'publish'); // 目标元素
     targetElement.appendChild(tempDiv);
     targetElement.scrollTop = targetElement.scrollHeight;
     while (targetElement.children.length > 30) {
         // 删除最前面的子元素
         targetElement.removeChild(targetElement.firstChild);
     }

     document.getElementById('copy-publish-topic-' + id).addEventListener('contextmenu', function(event) {
         event.preventDefault();
         navigator.clipboard.writeText(topic).then(function() {
             console.log('copy:' + topic);
         }).catch(function(error) {
             console.error('copy fail :', error);
         });
     });
     document.getElementById('copy-publish-payload-' + id).addEventListener('contextmenu', function(event) {
         event.preventDefault();
         navigator.clipboard.writeText(payload).then(function() {
             console.log('copy:' + payload);
         }).catch(function(error) {
             console.error('copy fail :', error);
         });
     });
 }


 async function init_receive_publish_item(trace_id, topic, payload, qos, ty, connect_id, time, byteStream) {
     let template = "<div></div><span id=\"copy-publish-topic-__id__\" class=\"mb-2 px-2 py-1 text-gray-800\">__topic__</span></div>\n" +
         "        <div id=\"copy-publish-payload-__id__\" class=\"mb-2 px-2 py-1 flex-grow rounded-lg bg-green-200 text-gray-800\">" +
         "          <p id='payload___id__' class='clamp-2 break-words'>__payload__</p></div>\n" +
         "        <div class=\"flex justify-end\"><span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full mr-2\">QoS __qos__</span>\n" +
         "        <span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full\"><select id='payload_ty___id__' class=\"bg-green-200 shadow rounded px-0.5\" >\n" +
         "                        <option __Text__>Text</option>\n" +
         "                        <option __Json__>Json</option>\n" +
         "                        <option __Hex__>Hex</option>\n" +
         "                    </select></span>\n" +
         "        <span class=\"px-2 py-1 ml-auto\">__time__</span></div>\n"
     ;

     let option_text = "";
     let option_json = "";
     let option_hex = "";
     if (ty === "Text") {
         option_text = "selected";
     } else if (ty === "Json") {
         option_json = "selected";
     } else if (ty === "Hex") {
         option_hex = "selected";
     }

     const htmlString = template.replaceAll("__id__", trace_id).replaceAll("__topic__", topic)
         .replaceAll("__payload__", payload)
         .replaceAll("__qos__", qos).replaceAll("__time__", time)
         .replaceAll("__Text__", option_text)
         .replaceAll("__Json__", option_json)
         .replaceAll("__Hex__", option_hex);

     var tempDiv = document.createElement('div');
     tempDiv.innerHTML = htmlString;
     tempDiv.className = "items-center justify-between p-2 bg-white rounded-lg shadow-md mb-2 max-w-72";

     var targetElement = document.getElementById(connect_id + 'publish'); // 目标元素
     targetElement.appendChild(tempDiv);
     targetElement.scrollTop = targetElement.scrollHeight;
     while (targetElement.children.length > 30) {
         // 删除最前面的子元素
         targetElement.removeChild(targetElement.firstChild);
     }

     document.getElementById('copy-publish-topic-' + trace_id).addEventListener('contextmenu', function(event) {
         event.preventDefault();
         navigator.clipboard.writeText(topic).then(function() {
             console.log('copy:' + topic);
         }).catch(function(error) {
             console.error('copy fail :', error);
         });
     });
     document.getElementById('payload_ty_' + trace_id).addEventListener('change', function(event) {
         event.preventDefault();
         let value = event.target.value;
         let payload_new = parse_payload(value, byteStream);
         document.getElementById('payload_' + trace_id).innerText = payload_new;
     });

     document.getElementById('copy-publish-payload-' + trace_id).addEventListener('contextmenu', function(event) {
         event.preventDefault();
         navigator.clipboard.writeText(payload).then(function() {
             console.log('copy:' + payload);
         }).catch(function(error) {
             console.error('copy fail :', error);
         });
     });
 }


 function init_subscribe_his_item(broker_id, topic, qos, ty) {
     let template = "        <div class=\"flex items-center pb-1 mb-2\">\n" +
         "            <span id=\"copy-subcribe-topic-__id__\" class=\"flex-grow text-gray-800\">__topic__</span>\n" +
         "        <a onclick='delete_subscribe_his(\"__topic__\", __qos__, \"__ty__\", __broker_id__)'><i class=\"ml-auto layui-icon layui-icon-close py-2 px-1 \"></i></a>\n" +
         "        </div>\n" +
         "        <span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full\">QoS __qos__</span>\n" +
         "        <span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full\">__ty__</span>\n"
     ;

     const htmlString = template.replaceAll("__topic__", topic)
         .replaceAll("__qos__", qos).replaceAll("__ty__", ty).replaceAll("__broker_id__", broker_id);

     var tempDiv = document.createElement('div');
     tempDiv.innerHTML = htmlString;
     tempDiv.className = "items-center justify-between p-2 bg-white rounded-lg shadow-md mb-4";
     tempDiv.addEventListener('dblclick', async function () {
         let trace_id = next_trace_id();
         init_subscribe_item(trace_id, topic, qos, ty, broker_id, get_time());
         var formObject = {};
         formObject["trace_id"] = trace_id;
         formObject["broker_id"] = broker_id;
         formObject["qos"] = get_qos(qos);
         formObject["topic"] = topic;
         formObject["payload_ty"] = ty;
         window.subscribes[broker_id][formObject["topic"]] = {
             ty: formObject["payload_ty"],
             topic: formObject["topic"]
         };
         let rs = await get_invoke()("subscribe", { datas : formObject});

         var tableBody = document.getElementById("subs_his_modal");
         tableBody.style.display = 'none';
         console.log(rs);
     });
     return tempDiv
 }

 function init_publish_his_item(broker_id, topic, qos, ty, payload, retain) {
     let template = "        <div class=\"flex items-center pb-1  \">\n" +
         "            <span class=\"flex-grow text-gray-800\">__topic__</span>\n" +
         "        <a onclick='delete_publish_his(\"__topic__\", __qos__, \"__ty__\", \"__payload__\", __retain__, __broker_id__)'><i class=\"ml-auto layui-icon layui-icon-close py-2 px-1 \"></i></a>\n" +
         "        </div>\n" +
         "        <div class=\"mb-2 px-2 py-1 flex-grow rounded-lg bg-green-200 text-gray-800\">" +
         "          <p class='clamp-2 break-words'>__payload__</p></div>\n" +
         "        <div class=\"flex\"><span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full mr-2\">QoS __qos__</span>\n" +
         "        <span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full mr-2\">__ty__</span>\n"
     ;
     if(retain) {
         template += "<span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full\">retain</span></div>\n";
     } else {
         template += "</div>\n";
     }

     const htmlString = template.replaceAll("__topic__", topic)
         .replaceAll("__payload__", payload)
         .replaceAll("__qos__", qos)
         .replaceAll("__retain__", retain).replaceAll("__ty__", ty).replaceAll("__broker_id__", broker_id);


     var tempDiv = document.createElement('div');
     tempDiv.innerHTML = htmlString;
     tempDiv.className = "items-center justify-between p-2 bg-white rounded-lg shadow-md mb-4";
     tempDiv.addEventListener('dblclick', async function () {
         let trace_id = next_trace_id();
         init_publish_item(trace_id, topic, payload, qos, ty, broker_id, get_time(), retain);
         var formObject = {};
         formObject["trace_id"] = trace_id;
         formObject["broker_id"] = broker_id;
         formObject["qos"] = get_qos(qos);
         formObject["topic"] = topic;
         formObject["payload_ty"] = ty;
         formObject["retain"] = retain;
         formObject["msg"] = payload;
         let rs = await get_invoke()("publish", { datas : formObject});

         var tableBody = document.getElementById("publish_his_modal");
         tableBody.style.display = 'none';
         console.log(rs);
     });
     return tempDiv
 }