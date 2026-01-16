document.addEventListener('DOMContentLoaded', () => {
    // Handle navigation
    document.body.addEventListener('click', e => {
        if (e.target.matches('[data-link]')) {
            e.preventDefault();
            navigateTo(e.target.href);
        }
    });

    // Handle initial load
    router();
});

// History API navigation
const navigateTo = url => {
    history.pushState(null, null, url);
    router();
};

// Simple router
const router = async () => {
    const routes = [
        { path: "/", view: Home },
        { path: "/users", view: Users },
        { path: "/about", view: About }
    ];

    // Test each route for potential match
    const potentialMatches = routes.map(route => {
        return {
            route: route,
            isMatch: location.pathname === route.path
        };
    });

    let match = potentialMatches.find(potentialMatch => potentialMatch.isMatch);

    if (!match) {
        match = {
            route: routes[0],
            isMatch: true
        };
    }

    const view = match.route.view();
    document.querySelector("#content").innerHTML = await view;
};

// Views
const Home = async () => {
    return `
        <h2>Welcome</h2>
        <p>This is a simple SPA integrated with a Rust backend.</p>
        <p>Try navigating to "Users" to fetch data from the API.</p>
    `;
};

const About = async () => {
    return `
        <h2>About</h2>
        <p>This demo showcases:</p>
        <ul>
            <li>Static file serving</li>
            <li>SPA routing fallback</li>
            <li>API integration</li>
        </ul>
    `;
};

const Users = async () => {
    try {
        const response = await fetch('/api/users');
        const users = await response.json();

        return `
            <h2>Users List (Fetched from API)</h2>
            <div class="users-list">
                ${users.map(user => `
                    <div class="user-card">
                        <h3>${user.username} <small>(${user.role})</small></h3>
                        <p>ID: ${user.id}</p>
                    </div>
                `).join('')}
            </div>
        `;
    } catch (error) {
        return `
            <h2>Error</h2>
            <p>Failed to load users: ${error}</p>
        `;
    }
};

// Handle back/forward buttons
window.addEventListener("popstate", router);
