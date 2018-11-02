let socket = new WebSocket('ws://localhost:8088/ws/');
socket.onopen = function (event) {
    socket.send('connect');
};
