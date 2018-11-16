﻿using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.CompilerServices;
using UnityEngine;
using UnityEngine.AddressableAssets;
using UnityEngine.ResourceManagement;
using UniRx.Async;

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
        // TODO: Handle serialization errors.
        var state = await _socket.RecvMessageAsync<GameStateData>();
        Debug.LogFormat("Recieved initial state: {0}", state);
        Debug.LogFormat("Received initial state with {0} players", state.Players.Count);

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
            if (player.PendingTurn.Movement.HasValue)
            {
                var movementPreview = await Addressables.Instantiate<GameObject>(_playerMovementPreviewPrefab);
                movementPreview.transform.localPosition = player.PendingTurn.Movement.Value.WorldPos;
            }
        }

        while (true)
        {
            // TODO: Handle an exception being thrown while waiting (i.e. if we disconnect).
            // TODO: Handle serialization errors.
            var update = await _socket.RecvMessageAsync<Message>();
            switch (update.Type)
            {
                case MessageType.PlayerAdded:
                    var playerAdded = update.Data.ToObject<PlayerAdded>();
                    // TODO: Add the player to the local world.
                    break;

                case MessageType.SetMovement:
                    var setMovement = update.Data.ToObject<SetMovement>();
                    // TODO: Update the movement preview for the specified player.
                    break;
            }
        }
    }

    private void OnDestroy()
    {
        _socket.Close();
    }
}
