

function check_to_bool(val) {
    return val === "on"
}

function get_time() {
    // 获取当前时间的Date对象
    var now = new Date();
    // 获取时间的各个部分
    var hours = now.getHours();
    var minutes = now.getMinutes();
    var seconds = now.getSeconds();
    var milliseconds = now.getMilliseconds();
// 格式化时间。确保每个时间部分都是两位数字，除了毫秒可能是三位数
    var formattedTime = hours.toString().padStart(2, '0') + ':' +
        minutes.toString().padStart(2, '0') + ':' +
        seconds.toString().padStart(2, '0') + '.' +
        milliseconds.toString().padStart(3, '0');

    console.log("当前时间（精确到毫秒）: " + formattedTime);
    return formattedTime
}