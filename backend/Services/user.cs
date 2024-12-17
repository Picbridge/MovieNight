using System;
using System.Security.Cryptography;
using System.Text;
using Azure.Cosmos;
using Microsoft.AspNetCore.Http;

public class UserService
{
    private readonly CosmosContainer _container;
    private readonly IHttpContextAccessor _httpContextAccessor;

    public UserService(CosmosContainer container, IHttpContextAccessor httpContextAccessor)
    {
        _container = container;
        _httpContextAccessor = httpContextAccessor ?? throw new ArgumentNullException(nameof(httpContextAccessor));
    }

    public async Task<User> CreateUserAsync(string id, string password)
    {
        if (string.IsNullOrEmpty(id) || string.IsNullOrEmpty(password))
        {
            throw new ArgumentNullException("ID or password is empty.");
        }
        Console.WriteLine($"ID: {id}, Password: {password}");

        var existingUser = await GetUserAsync(id);
        if (existingUser != null && existingUser.Id != null)
        {
            Console.WriteLine($"User {existingUser.Id} already exists.");
            throw new Exception("User already exists.");
        }

        var user = new User
        {
            Id = id, // Ensure this maps to the "id" field in Cosmos DB
            Password = HashPassword(password),
            Type = "user"
        };

        Console.WriteLine($"Payload: {System.Text.Json.JsonSerializer.Serialize(user)}");
        await _container.CreateItemAsync(user, new PartitionKey(user.Type)); // Ensure the partition key matches
        Console.WriteLine($"User {id} created.");

        return user;
    }

    public async Task<User> GetUserAsync(string id)
    {
        var query = new QueryDefinition("SELECT * FROM c WHERE c.id = @id AND c.type = @type")
            .WithParameter("@id", id)
            .WithParameter("@type", "user");

        var iterator = _container.GetItemQueryIterator<User>(query);
        await foreach (var result in iterator)
        {

            return result;
        }

        Console.WriteLine($"User {id} not found.");
        return new User();
    }

    public async Task<string> GetCurrentUserAsync()
    {
        Console.WriteLine("Getting current user.");
        var sessionKeys = _httpContextAccessor.HttpContext?.Session.Keys;
        if (sessionKeys != null)
        {
            Console.WriteLine($"Session keys: {string.Join(", ", sessionKeys)}");
        }

        var userId = _httpContextAccessor.HttpContext?.Session.GetString("UserId");
        if (string.IsNullOrEmpty(userId))
        {
            throw new Exception("User not found.");
        }
        return userId ?? throw new Exception("User not found.");
    }

    public async Task<string> LoginAsync(string id, string password)
    {
        if (string.IsNullOrEmpty(id) || string.IsNullOrEmpty(password))
            throw new ArgumentNullException("ID or password is empty.");

        var user = await GetUserAsync(id);
        if (user == null || user.Id != id || user.Password == null)
            throw new Exception("User not found.");

        if (!VerifyPassword(password, user.Password))
            throw new Exception("Invalid password.");

        // Store the user ID in the session temporarily before token generation is implemented
        var httpContext = _httpContextAccessor.HttpContext;
        if (httpContext == null)
            throw new Exception("HttpContext is not available.");

        if (httpContext.Session == null)
            throw new Exception("Session is not available.");

        // Store the user ID in the session
        httpContext.Session.SetString("UserId", user.Id);
        Console.WriteLine($"User {user.Id} logged in. Session initialized.");

        return GenerateToken(user); // Return a JWT token
    }

    public async Task LogoutAsync()
    {
        var httpContext = _httpContextAccessor.HttpContext;
        if (httpContext == null)
            throw new Exception("HttpContext is not available.");

        if (httpContext.Session == null)
            throw new Exception("Session is not available.");

        httpContext.Session.Remove("UserId");
        Console.WriteLine("User logged out. Session cleared.");
    }

    private string HashPassword(string password)
    {
        using var sha256 = SHA256.Create();
        var bytes = Encoding.UTF8.GetBytes(password);
        return Convert.ToBase64String(sha256.ComputeHash(bytes));
    }

    private bool VerifyPassword(string inputPassword, string hashedPassword)
    {
        var inputHash = HashPassword(inputPassword) ?? throw new ArgumentNullException(nameof(inputPassword));
        return inputHash == hashedPassword;
    }

    private string GenerateToken(User user)
    {
        // Temporary faux token
        return Convert.ToBase64String(Encoding.UTF8.GetBytes($"{user.Id}:{Guid.NewGuid()}"));
    }
}
