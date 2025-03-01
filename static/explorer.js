document.addEventListener('DOMContentLoaded', function() {
    document.addEventListener('click', function(event) {
        const folderLabel = event.target.closest('.folder-label');
        
        if (folderLabel) {
            // This is a click on a folder-label or one of its children
            console.log('Folder clicked:', folderLabel.querySelector('.folder-name').textContent);
            
            // Get the parent directory element and toggle its class
            const directory = folderLabel.closest('.directory');
            if (directory) {
                console.log('Toggling folder:', directory.classList.contains('folder-open') ? 'close' : 'open');
                directory.classList.toggle('folder-open');
            }
        }
    });
    
    const themeToggleBtn = document.getElementById('theme-toggle');
    const themeIcon = document.getElementById('theme-icon');
    const themeText = document.getElementById('theme-text');
    const htmlElement = document.documentElement;
    
    const savedTheme = localStorage.getItem('theme') || 'light';
    htmlElement.setAttribute('data-theme', savedTheme);
    updateThemeUI(savedTheme);
    
    themeToggleBtn.addEventListener('click', function() {
        const currentTheme = htmlElement.getAttribute('data-theme');
        const newTheme = currentTheme === 'light' ? 'dark' : 'light';
        
        htmlElement.setAttribute('data-theme', newTheme);
        localStorage.setItem('theme', newTheme);
        
        updateThemeUI(newTheme);
    });
    
    function updateThemeUI(theme) {
        if (theme === 'dark') {
            themeIcon.textContent = '☀️';
            themeText.textContent = 'Light Mode';
        } else {
            themeIcon.textContent = '🌙';
            themeText.textContent = 'Dark Mode';
        }
    }
    
    function expandToCurrentFile() {
        const currentPath = window.location.pathname;
        if (!currentPath.startsWith('/view/')) return;
        
        console.log('Current path:', currentPath);
        
        const activeFileLink = Array.from(document.querySelectorAll('.file-link')).find(
            link => link.getAttribute('href') === currentPath
        );
        
        if (activeFileLink) {
            console.log('Found active file:', activeFileLink.textContent);
            activeFileLink.classList.add('current-file');
            
            // Find all parent directories and open them
            let parent = activeFileLink.closest('.directory');
            while (parent) {
                console.log('Opening parent folder');
                parent.classList.add('folder-open');
                parent = parent.parentElement.closest('.directory');
            }
            
            // Scroll to make the file visible
            setTimeout(() => {
                activeFileLink.scrollIntoView({ block: 'center', behavior: 'smooth' });
            }, 300);
        }
    }
    
    // Run the expand function
    expandToCurrentFile();
});