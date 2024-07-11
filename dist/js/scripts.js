document.addEventListener("DOMContentLoaded", () => {
    const form = document.getElementById("weather-form");
    const weatherResults = document.getElementById("weather-results");

    form.addEventListener("submit", async (event) => {
        event.preventDefault();

        const latitude = document.getElementById("latitude").value;
        const longitude = document.getElementById("longitude").value;

        // Fetch weather data based on latitude and longitude
        const weatherData = await fetchWeatherData(latitude, longitude);

        displayWeatherResults(weatherData);
    });

    function fetchWeatherData(latitude, longitude) {
        // Placeholder URL - replace with your actual weather API endpoint
        const url = `https://api.example.com/weather?lat=${latitude}&lon=${longitude}`;

        return fetch(url)
            .then(response => response.json())
            .then(data => data);
    }

    function displayWeatherResults(data) {
        // Display weather data - update according to your API response
        weatherResults.innerHTML = `
            <h2>Weather Results</h2>
            <p>Temperature: ${data.temperature}°C</p>
            <p>Wind Speed: ${data.wind_speed} m/s</p>
        `;
    }

    document.querySelectorAll('nav a').forEach(link => {
        link.addEventListener('click', (event) => {
            event.preventDefault();
            const sectionId = event.target.getAttribute('href').substring(1);
            document.getElementById(sectionId).scrollIntoView({ behavior: 'smooth' });
        });
    });
});
