using Microsoft.AspNetCore.Builder;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.Hosting;
using Microsoft.AspNetCore.Http;
using System.Threading.Tasks;
using Azure.Cosmos;

class Program
{
    private static Task ServeHomePageDelegate(HttpContext context, IConfiguration configuration)
    {
        var _frontendUrl = configuration["Frontend:Url"] ?? "http://localhost:80";
        if (string.IsNullOrEmpty(_frontendUrl))
        {
            Console.WriteLine("Frontend URL is not configured.");
            context.Response.StatusCode = 500;
            return context.Response.WriteAsync("Frontend URL is not configured.");
        }

        Console.WriteLine($"Redirecting to {_frontendUrl}");
        context.Response.Redirect(_frontendUrl);
        return Task.CompletedTask;
    }

    static void Main(string[] args)
    {
        // Load configuration
        Config config = Config.LoadFromEnvironment();
        if (config == null)
        {
            Console.WriteLine("Failed to load configuration.");
            return;
        }

        WebApplicationBuilder builder = WebApplication.CreateBuilder(args);

        // Logging
        builder.Logging.AddConsole();

        // Add essential services
        builder.Services.AddControllers();
        builder.Services.AddHttpClient();
        builder.Services.AddDistributedMemoryCache(); 
        builder.Services.AddSession(options =>
        {
            options.IdleTimeout = TimeSpan.FromMinutes(30);
        });

        string frontendUrl = builder.Configuration["Frontend:Url"] ?? "http://localhost";


        builder.Services.AddCors(options =>
        {
            options.AddPolicy("AllowFrontend", builder =>
            {
                builder.WithOrigins("http://localhost", "http://localhost:8080", frontendUrl)
                    .AllowAnyHeader()
                    .AllowAnyMethod()
                    .AllowCredentials();
            });
        });

        builder.Services.AddSingleton<IHttpContextAccessor, HttpContextAccessor>();
        builder.Services.AddSingleton((serviceProvider) =>
        {
            var client = new CosmosClient(config.CosmosDbUrl, config.CosmosDbKey);
            Console.WriteLine("CosmosClient initialized.");

            var database = client.GetDatabase(config.DatabaseName);
            Console.WriteLine($"Database '{config.DatabaseName}' accessed.");

            var container = database.GetContainer(config.ContainerName);
            Console.WriteLine($"Container '{config.ContainerName}' accessed.");

            return container;
        });

        builder.Services.AddSingleton<RecommenderService>();
        builder.Services.AddSingleton<RecommendationService>();
        builder.Services.AddSingleton<UserService>();
        builder.Services.AddSingleton<ProfileService>();

        WebApplication app = builder.Build();

        // Middleware
        app.UseSession();
        app.UseCors("AllowFrontend");
        app.UseRouting();
        app.MapControllers();

        // Map routes
        app.MapGet("/", RedirectToHomeDelegate);
        app.MapGet("/home", (context) => ServeHomePageDelegate(context, app.Configuration));

        app.Run();
    }

    private static Task RedirectToHomeDelegate(HttpContext context)
    {
        Console.WriteLine("Redirecting to /home");
        context.Response.Redirect("/home");
        return Task.CompletedTask;
    }
}
