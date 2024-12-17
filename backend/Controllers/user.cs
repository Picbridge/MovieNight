using Microsoft.AspNetCore.Mvc;
using System;
using System.Collections.Generic;
using System.Threading.Tasks;

[ApiController]
[Route("api/[controller]")]
public class UserController : Controller
{
    private readonly UserService _userService;

    public UserController(UserService userService)
    {
        _userService = userService;
    }

    public class LoginRequest
    {
        public string? Id { get; set; }
        public string? Password { get; set; }
    }

    public class RegisterRequest
    {
        public string? Id { get; set; }
        public string? Password { get; set; }
    }

    [HttpGet("isvalid")]
    public async Task<IActionResult> GetCurrentUser()
    {
        try
        {
            var user = await _userService.GetCurrentUserAsync();
            return Ok(user);
        }
        catch (Exception ex)
        {
            return Unauthorized(new { Error = ex.Message });
        }
    }

    [HttpPost("login")]
    public async Task<IActionResult> Login([FromBody] LoginRequest request)
    {
        Console.WriteLine("Login endpoint hit.");
        try
        {
            if (string.IsNullOrEmpty(request.Id) || string.IsNullOrEmpty(request.Password))
            {
                return BadRequest(new { Error = "ID or password is empty." });
            }
            
            Console.WriteLine($"ID: {request.Id}, Password: {request.Password}");
            var token = await _userService.LoginAsync(request.Id, request.Password) ?? throw new ArgumentNullException(nameof(request.Password));
            return Ok(new { Token = token });
        }
        catch (Exception ex)
        {
            return Unauthorized(new { Error = ex.Message });
        }
    }

    [HttpPost("logout")]
    public async Task<IActionResult> Logout()
    {
        try
        {
            await _userService.LogoutAsync();
            return Ok();
        }
        catch (Exception ex)
        {
            return BadRequest(new { Error = ex.Message });
        }
    }
    
    [HttpPost("register")]
    public async Task<IActionResult> Register([FromBody] RegisterRequest request)
    {
        Console.WriteLine("Register endpoint hit.");
        try
        {
            if (string.IsNullOrEmpty(request.Id) || string.IsNullOrEmpty(request.Password))
            {
                return BadRequest(new { Error = "ID or password is empty." });
            }
            var user = await _userService.CreateUserAsync(request.Id, request.Password) ?? throw new ArgumentNullException(nameof(request.Password));
            
            Console.WriteLine($"User: {user}");
            return Ok(user);
        }
        catch (Exception ex)
        {
            return BadRequest(new { Error = ex.Message });
        }
    }
}

