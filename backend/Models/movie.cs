using System.Collections.Generic;
using System.Text.Json.Serialization;

public class Movie
{
    [JsonPropertyName("id")]
    public string? Id { get; set; } // Unique identifier for the movie

    [JsonPropertyName("title")]
    public string? Title { get; set; } // Title of the movie

    [JsonPropertyName("description")]
    public string? Description { get; set; } // Description of the movie

    [JsonPropertyName("imdb_rating")]
    public string? ImdbRating { get; set; } // IMDB rating

    [JsonPropertyName("stars")]
    public List<string>? Stars { get; set; } // List of stars in the movie

    [JsonPropertyName("image_url")]
    public string? ImageUrl { get; set; } // URL to the movie image

    [JsonPropertyName("released_year")]
    public string? ReleasedYear { get; set; } // Year of release

    [JsonPropertyName("runtime")]
    public string? Runtime { get; set; } // Runtime of the movie

    [JsonPropertyName("metadata")]
    public string? Metadata { get; set; } // Additional metadata or description

    [JsonPropertyName("genres")]
    public List<string>? Genres { get; set; } // List of genres for the movie

    [JsonPropertyName("director")]
    public string? Director { get; set; } // Director of the movie
}
