using DotNetEnv;

public class Config
{
    public string? CosmosDbUrl { get; private set; }
    public string? CosmosDbKey { get; private set; }
    public string? DatabaseName { get; private set; }
    public string? ContainerName { get; private set; }
    public string? RedisConnectionString { get; private set; }

    public string? FrontendUrl { get; private set; }

    public string? RecommenderUrl { get; private set; }
    
    // Static factory method to initialize Config from environment variables
    public static Config LoadFromEnvironment()
    {
        Env.Load();

        return new Config
        {
            CosmosDbUrl = Env.GetString("COSMOS_DB_URL") ?? throw new ArgumentNullException("COSMOS_DB_URL not found in environment."),
            CosmosDbKey = Env.GetString("COSMOS_DB_KEY") ?? throw new ArgumentNullException("COSMOS_DB_KEY not found in environment."),
            DatabaseName = Env.GetString("DATABASE_NAME") ?? throw new ArgumentNullException("DATABASE_NAME not found in environment."),
            ContainerName = Env.GetString("CONTAINER_NAME") ?? throw new ArgumentNullException("CONTAINER_NAME not found in environment."),
            RedisConnectionString = Env.GetString("REDIS_CACHE_CONNECTION") ?? throw new ArgumentNullException("REDIS_CACHE_CONNECTION not found in environment."),
            FrontendUrl = Env.GetString("FRONTEND_URL") ?? throw new ArgumentNullException("FRONTEND_URL not found in environment."),
            RecommenderUrl = Env.GetString("RECOMMENDER_URL") ?? throw new ArgumentNullException("RECOMMENDER_URL not found in environment.")
        };
    }
}
