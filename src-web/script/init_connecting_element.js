 async function init_subscribe_item(id, topic, qos, ty, connect_id, time) {
    let template = "        <div class=\"flex items-center pb-1 mb-2\">\n" +
        "            <span id=\"subcribe-status-__id__\" class=\"h-3 w-3 bg-gray-400 rounded-full mr-2\"></span>\n" +
        "            <span id=\"copy-subcribe-topic-__id__\" class=\"flex-grow text-gray-800\">__topic__</span>\n" +
        "        <a onclick='unsubscribe(__id__)'><i class=\"ml-auto layui-icon layui-icon-close py-2 px-1 \"></i></a>\n" +
        "        </div>\n" +
        "        <span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full\">QoS __qos__</span>\n" +
        "        <span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full\">__ty__</span>\n" +
     "        <span class=\"px-2 py-1 ml-auto\">__time__</span>\n"
     ;

     const htmlString = template.replaceAll("__id__", id).replaceAll("__topic__", topic)
         .replaceAll("__qos__", qos).replaceAll("__ty__", ty).replaceAll("__time__", time);

     var tempDiv = document.createElement('div');
     tempDiv.innerHTML = htmlString;
     tempDiv.className = "items-center justify-between p-2 bg-white rounded-lg shadow-md mb-4";
     tempDiv.addEventListener('dblclick', function () {
         unsubscribe(id);
     });

     var targetElement = document.getElementById(connect_id + 'subs'); // 目标元素
     targetElement.appendChild(tempDiv);

     document.getElementById('copy-subcribe-topic-' + id).addEventListener('contextmenu', function(event) {
         event.preventDefault();
         // todo 提示
         navigator.clipboard.writeText(topic).then(function() {
             console.log('copy:' + topic);
         }).catch(function(error) {
             console.error('copy fail :', error);
         });
     });
}

 async function init_publish_item(id, topic, payload, qos, ty, connect_id, time) {
     let template = "        <div class=\"flex items-center pb-1  \">\n" +
         "            <span id=\"publish-status-__id__\" class=\"h-3 w-3 bg-gray-400 rounded-full mr-2\"></span>\n" +
         "            <span id=\"copy-publish-topic-__id__\" class=\"flex-grow text-gray-800\">__topic__</span>\n" +
         "        </div>\n" +
         "        <div id=\"copy-publish-payload-__id__\" class=\"mb-2 px-2 py-1 flex-grow rounded-lg bg-green-200 text-gray-800\">" +
         "          <p class='clamp-2 break-words'>__payload__</p></div>\n" +
         "        <div class=\"flex justify-end\"><span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full mr-2\">QoS __qos__</span>\n" +
         "        <span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full\">__ty__</span>\n" +
         "        <span class=\"px-2 py-1 ml-auto\">__time__</span></div>\n"
     ;

     const htmlString = template.replaceAll("__id__", id).replaceAll("__topic__", topic)
         .replaceAll("__payload__", payload)
         .replaceAll("__qos__", qos).replaceAll("__ty__", ty).replaceAll("__time__", time);

     var tempDiv = document.createElement('div');
     tempDiv.innerHTML = htmlString;
     tempDiv.className = "items-center justify-between p-2 bg-white rounded-lg shadow-md mb-2 max-w-72 ml-auto";
     // tempDiv.addEventListener('dblclick', function () {
     //     unsubscribe(id);
     // });

     var targetElement = document.getElementById(connect_id + 'publish'); // 目标元素
     targetElement.appendChild(tempDiv);

     targetElement.scrollTop = targetElement.scrollHeight;

     document.getElementById('copy-publish-topic-' + id).addEventListener('contextmenu', function(event) {
         event.preventDefault();
         // todo 提示
         navigator.clipboard.writeText(topic).then(function() {
             console.log('copy:' + topic);
         }).catch(function(error) {
             console.error('copy fail :', error);
         });
     });
     document.getElementById('copy-publish-payload-' + id).addEventListener('contextmenu', function(event) {
         event.preventDefault();
         // todo 提示
         navigator.clipboard.writeText(payload).then(function() {
             console.log('copy:' + payload);
         }).catch(function(error) {
             console.error('copy fail :', error);
         });
     });
 }


 async function init_receive_publish_item(id, topic, payload, qos, ty, connect_id, time) {
     let template = "<div></div><span id=\"copy-publish-topic-__id__\" class=\"mb-2 px-2 py-1 text-gray-800\">__topic__</span></div>\n" +
         "        <div id=\"copy-publish-payload-__id__\" class=\"mb-2 px-2 py-1 flex-grow rounded-lg bg-green-200 text-gray-800\">" +
         "          <p class='clamp-2 break-words'>__payload__</p></div>\n" +
         "        <div class=\"flex justify-end\"><span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full mr-2\">QoS __qos__</span>\n" +
         "        <span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full\">__ty__</span>\n" +
         "        <span class=\"px-2 py-1 ml-auto\">__time__</span></div>\n"
     ;

     const htmlString = template.replaceAll("__id__", id).replaceAll("__topic__", topic)
         .replaceAll("__payload__", payload)
         .replaceAll("__qos__", qos).replaceAll("__ty__", ty).replaceAll("__time__", time);

     var tempDiv = document.createElement('div');
     tempDiv.innerHTML = htmlString;
     tempDiv.className = "items-center justify-between p-2 bg-white rounded-lg shadow-md mb-2 max-w-72";
     // tempDiv.addEventListener('dblclick', function () {
     //     unsubscribe(id);
     // });

     var targetElement = document.getElementById(connect_id + 'publish'); // 目标元素
     targetElement.appendChild(tempDiv);
     targetElement.scrollTop = targetElement.scrollHeight;

     document.getElementById('copy-publish-topic-' + id).addEventListener('contextmenu', function(event) {
         event.preventDefault();
         // todo 提示
         navigator.clipboard.writeText(topic).then(function() {
             console.log('copy:' + topic);
         }).catch(function(error) {
             console.error('copy fail :', error);
         });
     });
     document.getElementById('copy-publish-payload-' + id).addEventListener('contextmenu', function(event) {
         event.preventDefault();
         // todo 提示
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
         "        <a onclick='delete_subscribe_his(__topic__, __qos__, __ty__)'><i class=\"ml-auto layui-icon layui-icon-close py-2 px-1 \"></i></a>\n" +
         "        </div>\n" +
         "        <span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full\">QoS __qos__</span>\n" +
         "        <span class=\"px-2 py-1 text-green-800 bg-green-200 rounded-full\">__ty__</span>\n"
     ;

     const htmlString = template.replaceAll("__topic__", topic)
         .replaceAll("__qos__", qos).replaceAll("__ty__", ty);

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
         let rs = await get_invoke()("subscribe", { datas : formObject});
         console.log(rs);
     });
     return tempDiv
 }




 function init_publish_his_item(broker_id, topic, qos, ty, payload, retain) {
     let template = "        <div class=\"flex items-center pb-1  \">\n" +
         "            <span class=\"flex-grow text-gray-800\">__topic__</span>\n" +
         "        <a onclick='delete_publish_his(__topic__, __qos__, __ty__, __payload__)'><i class=\"ml-auto layui-icon layui-icon-close py-2 px-1 \"></i></a>\n" +
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
         .replaceAll("__qos__", qos).replaceAll("__ty__", ty);


     var tempDiv = document.createElement('div');
     tempDiv.innerHTML = htmlString;
     tempDiv.className = "items-center justify-between p-2 bg-white rounded-lg shadow-md mb-4";
     tempDiv.addEventListener('dblclick', async function () {
         let trace_id = next_trace_id();
         init_publish_item(trace_id, topic, payload, qos, ty, broker_id, get_time());
         var formObject = {};
         formObject["trace_id"] = trace_id;
         formObject["broker_id"] = broker_id;
         formObject["qos"] = get_qos(qos);
         formObject["topic"] = topic;
         formObject["payload_ty"] = ty;
         formObject["retain"] = retain;
         formObject["msg"] = payload;
         let rs = await get_invoke()("publish", { datas : formObject});
         console.log(rs);
     });
     return tempDiv
 }