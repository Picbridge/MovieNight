using System.Collections.Generic;
using System.Text.Json.Serialization;

public class Recommendation
{
    [JsonPropertyName("id")]
    public string? RecommendationId { get; set; } 
    [JsonPropertyName("user_id")]
    public string? UserId { get; set; }
    [JsonPropertyName("movies")]
    public List<Movie>? Movies { get; set; } 
    [JsonPropertyName("created_at")]
    public DateTime? CreatedAt { get; set; }
    [JsonPropertyName("type")]
    public string? Type { get; set; }
}
