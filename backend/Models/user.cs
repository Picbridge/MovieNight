using System.Text.Json.Serialization;

public class User
{
    [JsonPropertyName("id")] // Ensures this field is serialized as "id"
    public string? Id { get; set; }

    [JsonPropertyName("password")]
    public string? Password { get; set; }

    [JsonPropertyName("type")]
    public string? Type { get; set; }
}
