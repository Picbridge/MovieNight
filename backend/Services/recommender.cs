using System.Net.Http;
using System.Text;
using System.Text.Json;
using System.Threading.Tasks;
using Azure.Cosmos;
using System;

public class RecommenderService
{
    private readonly HttpClient _httpClient;
    private readonly string _recommenderUrl;
    private readonly CosmosContainer _container;

    public RecommenderService(HttpClient httpClient, IConfiguration configuration, CosmosContainer container)
    {
        _httpClient = httpClient;
        _recommenderUrl = configuration["RecommenderService:Url"] ?? "http://localhost:5000/";
        _container = container;
    }

    public async Task<List<Movie>> GetAllMoviesAsync()
    {
        Console.WriteLine("Fetching all movies");

        var query = new QueryDefinition("SELECT * FROM c WHERE c.type = @type")
            .WithParameter("@type", "movie");

        var iterator = _container.GetItemQueryIterator<Movie>(query);

        List<Movie> movies = new List<Movie>();

        // Collect all movies into a list
        await foreach (var movie in iterator)
        {
            movies.Add(movie);
        }

        

        return movies;
    }

    public async Task<Movie> GetRandomMovieAsync()
    {
        Console.WriteLine("Fetching random movie recommendation");

        var movies = await GetAllMoviesAsync();

        if (movies.Count == 0)
        {
            throw new Exception("No movies found.");
        }
        // Generate a random index
        var random = new Random();
        int randomIndex = random.Next(movies.Count);

        Console.WriteLine($"Random index: {movies[randomIndex].Title}");
        // Return the movie at the random index
        return movies[randomIndex];
    }


    public async Task<List<Movie>> GetRecommendationsAsync(StringContent content)
    {
        Console.WriteLine("Fetching recommendations");

        if (content == null)
        {
            throw new ArgumentNullException("Request is null.");
        }
        
        var url= _recommenderUrl + "/recommend";
        var response = await _httpClient.PostAsync(url, content);

        if (response.IsSuccessStatusCode)
        {
            Console.WriteLine("Recommendations fetched successfully.");
            var responseString = await response.Content.ReadAsStringAsync();
            
            List<Movie>? movies = JsonSerializer.Deserialize<List<Movie>>(responseString) ?? new List<Movie>();

            if (movies != null)
            {
                foreach (var movie in movies)
                {
                    Console.WriteLine($"Title: {movie.Title}, Year: {movie.ReleasedYear}");
                }
            }
            else
            {
                Console.WriteLine("No movies were deserialized.");
            }
            return movies ?? new List<Movie>();
        }

        throw new Exception($"Failed to fetch recommendations: {response.StatusCode}");
    }

    public async Task<string> ReasoningAsync(StringContent content)
    {
        Console.WriteLine("Fetching reasoning");

        if (content == null)
        {
            throw new ArgumentNullException("Request is null.");
        }
        
        var url = _recommenderUrl + "/reasoning";
        var response = await _httpClient.PostAsync(url, content);

        if (response.IsSuccessStatusCode)
        {
            var responseString = await response.Content.ReadAsStringAsync();
            string jsonString = JsonSerializer.Serialize(responseString);

            //Console.WriteLine($"Response: {jsonString}");
            return jsonString;
        }

        throw new Exception($"Failed to fetch reasoning: {response.StatusCode}");
    }

    public async Task<string> GetWhereToWatchAsync(StringContent content)
    {
        Console.WriteLine("Fetching where to watch");

        if (content == null)
        {
            throw new ArgumentNullException("Request is null.");
        }

        var url = _recommenderUrl + "/generate";
        var response = await _httpClient.PostAsync(url, content);

        if (response.IsSuccessStatusCode)
        {
            var responseString = await response.Content.ReadAsStringAsync();
            Console.WriteLine($"Response: {responseString}");

            // Parse the JSON to extract the "response" field
            using (var document = JsonDocument.Parse(responseString))
            {
                if (document.RootElement.TryGetProperty("response", out var responseElement))
                {
                    var responseValue = responseElement.GetString(); // This is the raw string
                    
                    if (!string.IsNullOrEmpty(responseValue))
                    {
                        // Clean up and return the processed string
                        var cleanedResponse = responseValue.Replace("[end of text]", "").Trim();
                        Console.WriteLine($"Cleaned Response: {cleanedResponse}");
                        return cleanedResponse;
                    }
                }
            }

            Console.WriteLine("No valid response field found.");
            return "No OTT platforms available.";
        }

        throw new Exception($"Failed to fetch where to watch: {response.StatusCode}");
    }
}