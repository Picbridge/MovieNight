using Microsoft.AspNetCore.Mvc;
using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using System.Text.Json.Serialization;
using System.Text.Json;
using System.Text;

[ApiController]
[Route("api/[controller]")]
public class RecommenderController : Controller
{
    private readonly RecommenderService _recommenderService;
    
    public RecommenderController(RecommenderService recommenderService)
    {
        _recommenderService = recommenderService;
    }

    public class ReasoningRequest
    {
        [JsonPropertyName("selected_movie")]
        public List<string>? SelectedMovies { get; set; }
        [JsonPropertyName("recommended_movie")]
        public List<string>? RecommendedMovies { get; set; }
    }
    public class RecommendRequest
    {
        [JsonPropertyName("movies")]
        public List<Movie>? Movies { get; set; }

        [JsonPropertyName("year_range")]
        public int[]? YearRange { get; set; } // Represented as an array of two integers

        [JsonPropertyName("runtime_range")]
        public int[]? RuntimeRange { get; set; } // Represented as an array of two integers

        [JsonPropertyName("rating")]
        public float? Rating { get; set; } // Represented as a single float value
    }

    public class MovieTitle
    {
        [JsonPropertyName("title")]
        public string? Title { get; set; }
    }

    [HttpPost("reasoning")]
    public async Task<IActionResult> Reasoning([FromBody] ReasoningRequest request)
    {
        try
        {
            Console.WriteLine("Reasoning endpoint hit.");
            if (request == null || request.SelectedMovies == null || request.RecommendedMovies == null)
            {
                return BadRequest("Selected and recommended movies are required.");
            }
            
            var content = new StringContent(JsonSerializer.Serialize(request), Encoding.UTF8, "application/json");
            var reasoning = await _recommenderService.ReasoningAsync(content);
            return Ok(reasoning);
        }
        catch (Exception ex)
        {
            return StatusCode(500, new { error = ex.Message });
        }
    }

    [HttpGet("random")]
    public async Task<IActionResult> GetRandomMovie()
    {
        try
        {
            var movie = await _recommenderService.GetRandomMovieAsync();
            return Ok(movie); 
        }
        catch (Exception ex)
        {
            return StatusCode(500, new { error = ex.Message });
        }
    }

    [HttpPost("recommend")]
    public async Task<IActionResult> GetRecommendations([FromBody] RecommendRequest request)
    {
        try
        {
            Console.WriteLine("Recommend endpoint hit.");

            if (request == null)
            {
                return BadRequest("Movie name is required.");
            }

            var content = new StringContent(JsonSerializer.Serialize(request), Encoding.UTF8, "application/json");
            var recommendations = await _recommenderService.GetRecommendationsAsync(content);

            return Ok(recommendations);
        }
        catch (Exception ex)
        {
            return StatusCode(500, new { error = ex.Message });
        }
    }

    [HttpPost("where")]
    public async Task<IActionResult> GetWhereToWatch([FromBody] MovieTitle request)
    {
        try
        {
            Console.WriteLine("Where endpoint hit.");

            if (request == null)
            {
                return BadRequest("Movie name is required.");
            }

            var content = new StringContent(JsonSerializer.Serialize(request), Encoding.UTF8, "application/json");
            var services = await _recommenderService.GetWhereToWatchAsync(content);

            return Ok(services);
        }
        catch (Exception ex)
        {
            return StatusCode(500, new { error = ex.Message });
        }
    }

}