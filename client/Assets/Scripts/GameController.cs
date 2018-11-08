using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class GameController : MonoBehaviour
{
    private WebSocket _socket;

    public IEnumerator Start()
    {
        _socket = new WebSocket(new Uri("ws://localhost:8088/ws/"));
        yield return _socket.Connect();
        if (!string.IsNullOrEmpty(_socket.error))
        {
            Debug.LogErrorFormat(this, "Failed to connect to server: {0}", _socket.error);

            // TODO: Provide some way for the user to retry estabilishing a connection.
            yield break;
        }

        Debug.Log("Websocket connection established!");

        // TODO: Wait to receive the initial game state from the server.
    }

    private void OnDestroy()
    {
        _socket.Close();
    }
}
