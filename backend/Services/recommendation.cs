using System.Net.Http;
using System.Text;
using System.Text.Json;
using System.Threading.Tasks;
using Azure.Cosmos;

public class RecommendationService
{
    private readonly CosmosContainer _container;

    public RecommendationService(CosmosContainer container)
    {
        _container = container;
    }

    public async Task<Recommendation> CreateRecommendationAsync(string userId, List<Movie> movies)
    {
        var recommendation = new Recommendation
        {
            RecommendationId = Guid.NewGuid().ToString(),
            UserId = userId,
            Movies = movies,
            CreatedAt = DateTime.UtcNow,
            Type = "recommendation"
        };

        await _container.CreateItemAsync(recommendation, new PartitionKey("recommendation"));
        return recommendation;
    }

    public async Task<List<Recommendation>> GetRecommendationsAsync(string userId)
    {
        var query = new QueryDefinition("SELECT * FROM c WHERE c.user_id = @userId AND c.type = 'recommendation'")
            .WithParameter("@userId", userId);

        var iterator = _container.GetItemQueryIterator<Recommendation>(query);

        var recommendations = new List<Recommendation>();
        await foreach (var recommendation in iterator)
        {
            recommendations.Add(recommendation);
        }

        return recommendations;
    }
}
