using Newtonsoft.Json;
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
        string initString = null;
        do
        {
            initString = _socket.RecvString();
        }
        while (initString == null);

        Debug.LogFormat(this, "Got init string: {0}", initString);

        var initialState = JsonConvert.DeserializeObject<GameStateData>(initString);
        Debug.LogFormat("Got initial state: {0}", initialState);
    }

    private void OnDestroy()
    {
        _socket.Close();
    }
}

public class GameStateData
{
    [JsonProperty("players")]
    public PlayerData[] Players { get; } = {};
}

public class PlayerData {
    [JsonProperty("pos")]
    public Vector2Int Pos { get; }

    [JsonProperty("health")]
    public HealthData Health { get; }

    [JsonProperty("pending_turn")]
    public TurnData PendingTurn { get; }
}

public class HealthData {
    [JsonProperty("max")]
    public int Max { get; }

    [JsonProperty("current")]
    public int Current { get; }
}

public class TurnData {
    [JsonProperty("movement")]
    public Vector2Int? Movement { get; }
}
