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

        // Create a player instance in the scene for each of the players that already exists
        // when we connect to the server.
        foreach (var player in state.Players)
        {
            var playerInstance = Instantiate(_playerPrefab);
            playerInstance.transform.localPosition = player.Pos.WorldPos;
        }
    }

    private void OnDestroy()
    {
        _socket.Close();
    }
}
