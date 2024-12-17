public class Profile
{
    public string? UserId { get; set; } // Link to the User entity
    public List<string> FavoriteDirectors { get; set; } = new List<string>();
    public List<string> Genres { get; set; } = new List<string>();
    public List<string> FavoriteActors { get; set; } = new List<string>();
    public List<string> FavoriteMovies { get; set; } = new List<string>();
    
    public string? Type { get; set; }
}
