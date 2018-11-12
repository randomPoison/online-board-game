using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;
using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class GameController : MonoBehaviour
{
    [SerializeField]
    private GameObject _playerPrefab = null;

    [SerializeField]
    private GameObject _playerMovementPreviewPrefab = null;

    private WebSocket _socket;

    private IEnumerator Start()
    {
        Debug.Assert(_playerPrefab != null, "Player prefab wasn't setup");

        _socket = new WebSocket(new Uri("ws://localhost:8088/ws/"));
        yield return _socket.Connect();
        if (!string.IsNullOrEmpty(_socket.error))
        {
            Debug.LogErrorFormat(this, "Failed to connect to server: {0}", _socket.error);

            // TODO: Provide some way for the user to retry estabilishing a connection.
            yield break;
        }

        Debug.Log("Websocket connection established!");

        // Wait to receive the initial game state from the server.
        var initString = _socket.RecvString();
        while (initString == null)
        {
            // Wait a frame before checking again.
            yield return null;

            initString = _socket.RecvString();
        }

        Debug.LogFormat(this, "Got init string: {0}", initString);

        // TODO: Handle serialization errors.
        var state = JsonConvert.DeserializeObject<GameStateData>(initString);

        // Create objects in the world as necessary based on the initial game state
        // when we first connect to the server.
        foreach (var player in state.Players)
        {
            // Create an object in the world for the player and set it to the world position
            // that corresponds to their grid position.
            var playerInstance = Instantiate(_playerPrefab);
            playerInstance.transform.localPosition = player.Pos.WorldPos;

            // Visualize the pending move action for the player, if they already have
            // one setup.
            if (player.PendingTurn.Movement.HasValue) {
                var movementPreview = Instantiate(_playerMovementPreviewPrefab);
                movementPreview.transform.localPosition = player.PendingTurn.Movement.Value.WorldPos;
            }
        }
    }

    private void OnDestroy()
    {
        _socket.Close();
    }
}
