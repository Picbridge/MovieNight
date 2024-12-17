using Azure.Cosmos;

public class ProfileService
{
    private readonly CosmosContainer _container;

    public ProfileService(CosmosContainer container)
    {
        _container = container;
    }

    public async Task<Profile> CreateOrUpdateProfileAsync(UpdateProfileRequest request)
    {
        var profile = new Profile
        {
            UserId = request.UserId ?? throw new ArgumentNullException(nameof(request.UserId)),
            FavoriteDirectors = request.FavoriteDirectors ?? throw new ArgumentNullException(nameof(request.FavoriteDirectors)),
            Genres = request.Genres ?? throw new ArgumentNullException(nameof(request.Genres)),
            FavoriteActors = request.FavoriteActors ?? throw new ArgumentNullException(nameof(request.FavoriteActors)),
            Type = "profile"
        };

        await _container.UpsertItemAsync(profile, new PartitionKey("profile"));
        return profile;
    }

    public async Task<Profile> GetProfileAsync(string userId)
    {
        var query = new QueryDefinition("SELECT * FROM c WHERE c.user_id = @userId AND c.type = 'profile'")
            .WithParameter("@userId", userId);

        var iterator = _container.GetItemQueryIterator<Profile>(query);

        await foreach (var profile in iterator)
        {
            return profile;
        }

        throw new Exception("Profile not found.");
    }
}
