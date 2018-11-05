let socket = new WebSocket('ws://localhost:8088/ws/');

socket.onmessage = function (event) {
    let payload = JSON.parse(event.data);
    console.log('Received message:', payload);
}

socket.onclose = function (event) {
    console.log('Disconnected from server:', event);
}
