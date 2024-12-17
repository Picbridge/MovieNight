using Microsoft.AspNetCore.Mvc;
using System;
using System.Collections.Generic;
using System.Threading.Tasks;

[ApiController]
[Route("api/[controller]")]
public class ProfileController : Controller
{
    private readonly ProfileService _profileService;

    public ProfileController(ProfileService profileService)
    {
        _profileService = profileService;
    }

    [HttpGet("{id}")]
    public async Task<IActionResult> GetProfile(string id)
    {
        try
        {
            var profile = await _profileService.GetProfileAsync(id);
            return Ok(profile);
        }
        catch (Exception ex)
        {
            return StatusCode(500, new { error = ex.Message });
        }
    }

    [HttpPost("{id}")]
    public async Task<IActionResult> UpdateProfile(string id, [FromBody] UpdateProfileRequest request)
    {
        try
        {
            var profile = await _profileService.CreateOrUpdateProfileAsync(request);
            return Ok(profile);
        }
        catch (Exception ex)
        {
            return StatusCode(500, new { error = ex.Message });
        }
    }
}

public class UpdateProfileRequest
{
    public string? UserId { get; set; }
    public List<string>? FavoriteDirectors { get; set; }
    public List<string>? Genres { get; set; }
    public List<string>? FavoriteActors { get; set; }
}
