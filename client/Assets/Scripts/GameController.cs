using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.CompilerServices;
using UnityEngine;
using UnityEngine.AddressableAssets;
using UnityEngine.ResourceManagement;
using UniRx.Async;
using static UniRx.Async.UnityAsyncExtensions;

public class GameController : MonoBehaviour
{
    [SerializeField]
    private AssetReference _playerPrefab = null;

    [SerializeField]
    private AssetReference _playerMovementPreviewPrefab = null;

    private WebSocket _socket;

    private async void Start()
    {
        Debug.Assert(_playerPrefab != null, "Player prefab wasn't setup");

		// TODO: Handle an exception being thrown as a result of the connection failing.
        _socket = await WebSocket.ConnectAsync(new Uri("ws://localhost:8088/ws/"));

		// Wait for the initial game state to come in from the server.
		//
		// TODO: Handle an exception being thrown while waiting (i.e. if we disconnect).
		var initString = await _socket.RecvStringAsync();
        Debug.LogFormat(this, "Got init string: {0}", initString);

        // TODO: Handle serialization errors.
        var state = JsonConvert.DeserializeObject<GameStateData>(initString);

        // Create objects in the world as necessary based on the initial game state
        // when we first connect to the server.
        foreach (var player in state.Players)
        {
            // Create an object in the world for the player and set it to the world position
            // that corresponds to their grid position.
            var playerInstance = await Addressables.Instantiate<GameObject>(_playerPrefab);
            playerInstance.transform.localPosition = player.Pos.WorldPos;

            // Visualize the pending move action for the player, if they already have
            // one setup.
            if (player.PendingTurn.Movement.HasValue) {
                var movementPreview = await Addressables.Instantiate<GameObject>(_playerMovementPreviewPrefab);
                movementPreview.transform.localPosition = player.PendingTurn.Movement.Value.WorldPos;
            }
        }
    }

    private void OnDestroy()
    {
        _socket.Close();
    }
}
