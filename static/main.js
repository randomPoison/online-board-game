window.onload = function (event) {
    let app = new Vue({
        el: '#app',

        data: {
            socket: null,
            players: [],
        },

        created: function () {
            this.socket = new WebSocket('ws://localhost:8088/ws/');

            this.socket.onmessage = (event) => {
                let payload = JSON.parse(event.data);
                console.log('Received message:', payload);

                // If the payload updates the list of players, apply the update
                // to the local state.
                if (payload.players != null) {
                    this.players = payload.players;
                }
            }

            this.socket.onclose = (event) => {
                console.log('Disconnected from server:', event);
            }
        }
    });
}
