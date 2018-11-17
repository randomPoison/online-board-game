let app = new Vue({
    el: '#app',

    data: {
        socket: null,
        players: {},
        movement: {},
    },

    created: function () {
        this.socket = new WebSocket('ws://localhost:8088/ws/');

        let onUpdateMessage = event => {
            let payload = JSON.parse(event.data);
            console.log('Received update message:', payload);

            this.players = payload.players;
        };

        this.socket.onmessage = event => {
            let payload = JSON.parse(event.data);
            console.log('Received init message:', payload);

            this.socket.onmessage = onUpdateMessage;
        };

        this.socket.onclose = (event) => {
            console.log('Disconnected from server:', event);
        }
    },

    methods: {
        submitMovement: function () {
            let payload = {
                message: 'MoveTo',
                pos: { x: this.movement.x, y: this.movement.y },
            };
            this.socket.send(JSON.stringify(payload));
            this.movement = {};
        }
    },
});
