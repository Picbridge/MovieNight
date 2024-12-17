using Microsoft.AspNetCore.Mvc;
using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using System.Text.Json.Serialization;
using System.Text.Json;

[ApiController]
[Route("api/[controller]")]
public class RecommendationController : Controller
{
    private readonly RecommendationService _recommendationService;

    public RecommendationController(RecommendationService recommendationService)
    {
        _recommendationService = recommendationService;
    }

    public class RecommendRequest
    {
        [JsonPropertyName("user_id")]
        public string? UserId { get; set; }
        [JsonPropertyName("movies")]
        public List<Movie>? Movies { get; set; }
    }

    [HttpPost("pull")]
    public async Task<IActionResult> GetRecommendations([FromBody] RecommendRequest request)
    {
        try
        {
            if (request == null || request.UserId == null)
            {
                return BadRequest("User ID is required.");
            }

            var recommendations = await _recommendationService.GetRecommendationsAsync(request.UserId);
            return Ok(recommendations);
        }
        catch (Exception ex)
        {
            return StatusCode(500, new { error = ex.Message });
        }
    }

    [HttpPost("push")]
    public async Task<IActionResult> CreateRecommendation([FromBody] RecommendRequest request)
    {
        Console.WriteLine("Recommendation endpoint hit.");
        try
        {
            if (request == null || request.Movies == null || request.UserId == null)
            {
                return BadRequest("Movie name is required.");
            }

            var recommendation = await _recommendationService.CreateRecommendationAsync(request.UserId, request.Movies);
            return Ok(recommendation);
        }
        catch (Exception ex)
        {
            return StatusCode(500, new { error = ex.Message });
        }
    }
}