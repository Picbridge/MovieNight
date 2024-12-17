# Base stage (ASP.NET Core runtime)
FROM mcr.microsoft.com/dotnet/aspnet:8.0 AS base
WORKDIR /app
EXPOSE 5130
ENV ASPNETCORE_URLS=http://+:5130

# Build stage (SDK for building)
FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
WORKDIR /src
COPY ["backend.csproj", "./"]
RUN dotnet restore "backend.csproj"
COPY . . 
RUN dotnet build "backend.csproj" -c Release -o /app/build

# Publish stage
FROM build AS publish
RUN dotnet publish "backend.csproj" -c Release -o /app/publish

# Final stage (runtime)
FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "backend.dll"]
