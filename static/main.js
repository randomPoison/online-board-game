let app = new Vue({
    el: '#app',

    data: {
        socket: null,
        players: [],
        movement: {},
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
