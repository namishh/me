const searchModal = document.getElementById('search-modal');
const closeSearch = document.getElementById('close-search');
const searchInput = document.getElementById('search-input');
const searchButton = document.getElementById('search-button');
const searchResults = document.getElementById('search-results');

document.querySelectorAll('.search-toggle').forEach(el => {
    el.addEventListener('click', () => {
        searchModal.classList.remove('hidden');
        searchInput.focus();
    });
});

closeSearch.addEventListener('click', () => {
    searchModal.classList.add('hidden');
});

searchModal.addEventListener('click', (e) => {
    if (e.target === searchModal) {
        searchModal.classList.add('hidden');
    }
});

document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape' && !searchModal.classList.contains('hidden')) {
        searchModal.classList.add('hidden');
    }
});

searchButton.addEventListener('click', () => {
    const query = searchInput.value.trim();
    if (query) {
        performSearch(query);
    }
});

searchInput.addEventListener('keypress', (e) => {
    if (e.key === 'Enter') {
        const query = searchInput.value.trim();
        if (query) {
            performSearch(query);
        }
    }
});

function performSearch(query) {
    fetch(`/api/search?q=${encodeURIComponent(query)}`)
        .then(response => response.json())
        .then(data => {
            displayResults(data);
        })
        .catch(error => {
            console.error('Error fetching search results:', error);
            searchResults.innerHTML = '<p class="text-red-500 font-mono text-xs">An error occurred while searching. Please try again.</p>';
        });
}

// Function to display search results
function displayResults(data) {
    searchResults.innerHTML = '';
    if (data.length === 0) {
        searchResults.innerHTML = '<p>No results found.</p>';
        return;
    }
    data.forEach(item => {
        console.log(item)
        const resultDiv = document.createElement('div');
        resultDiv.className = 'mb-4';

        const titleLink = document.createElement('a');
        titleLink.href = '/' + item.url;
        titleLink.className = 'font-semibold text-normal';
        titleLink.textContent = item.title;

        const titleH3 = document.createElement('h3');
        titleH3.appendChild(titleLink);

        resultDiv.appendChild(titleH3);

        item.contexts.forEach(context => {
            console.log(context)
            const contextDiv = document.createElement('a');
            contextDiv.href = '/' + context.url; 
            contextDiv.className = 'mt-2 p-2 border-[1px] hover:bg-neutral-100 block dark:hover:bg-neutral-800 border-neutral-400 dark:border-neutral-600';

            const contextP = document.createElement('p');
            contextP.className = 'whitespace-pre-line text-sm text-neutral-700 dark:text-neutral-300';
            contextP.textContent = context.context;

            contextDiv.appendChild(contextP);
            resultDiv.appendChild(contextDiv);
        });

        searchResults.appendChild(resultDiv);
    });
}