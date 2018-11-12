using System.Collections.Generic;
using Newtonsoft.Json;
using UnityEngine;

public class GameStateData
{
    [JsonProperty("players")]
    [JsonRequired]
    public List<PlayerData> Players { get; private set; }
}

public class PlayerData
{
    [JsonProperty("pos")]
    public GridPos Pos { get; private set; }

    [JsonProperty("health")]
    public HealthData Health { get; private set; }

    [JsonProperty("pending_turn")]
    public TurnData PendingTurn { get; private set; }
}

public class HealthData
{
    [JsonProperty("max")]
    public int Max { get; private set; }

    [JsonProperty("current")]
    public int Current { get; private set; }
}

public class TurnData
{
    [JsonProperty("movement")]
    public GridPos? Movement { get; private set; }
}

public struct GridPos
{
    public int x;

    public int y;

    public Vector3Int WorldPos
    {
        get { return new Vector3Int(x, 0, y); }
    }
}
