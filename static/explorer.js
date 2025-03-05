document.addEventListener('DOMContentLoaded', function() {
    document.addEventListener('click', function(event) {
        const folderLabel = event.target.closest('.folder-label');
        
        if (folderLabel) {
            const directory = folderLabel.closest('.directory');
            if (directory) {
                const folderContents = directory.querySelector('.folder-contents');
                const toggleIcon = directory.querySelector('.toggle-icon');
                
                if (folderContents.classList.contains('hidden')) {
                    folderContents.classList.remove('hidden');
                    folderContents.classList.add('block');
                    toggleIcon.classList.add('rotate-90');
                } else {
                    folderContents.classList.remove('block');
                    folderContents.classList.add('hidden');
                    toggleIcon.classList.remove('rotate-90');
                }
            }
        }
    });
    
    function setupThemeToggle(toggleElementId) {
        const themeToggleBtn = document.getElementById(toggleElementId);
        if (themeToggleBtn) {
            const htmlElement = document.documentElement;
            
            const savedTheme = localStorage.getItem('theme') || 'light';
            htmlElement.setAttribute('data-theme', savedTheme);
            updateThemeUI(savedTheme, themeToggleBtn);
            
            themeToggleBtn.addEventListener('click', function() {
                const currentTheme = htmlElement.getAttribute('data-theme');
                const newTheme = currentTheme === 'light' ? 'dark' : 'light';
                
                htmlElement.setAttribute('data-theme', newTheme);
                localStorage.setItem('theme', newTheme);
                
                updateThemeUI(newTheme, themeToggleBtn);
            });
        }
    }
    
    function updateThemeUI(theme, toggleBtn) {
        if (theme === 'dark') {
            toggleBtn.classList.add('ph-sun');
            toggleBtn.classList.remove('ph-moon-stars');
        } else {
            toggleBtn.classList.add('ph-moon-stars');
            toggleBtn.classList.remove('ph-sun');
        }
    }
    
    setupThemeToggle('theme-toggle');
    setupThemeToggle('mobile-theme-toggle');
    
    const mobileMenuToggle = document.getElementById('mobile-menu-toggle');
    const sidebar = document.getElementById('sidebar');
    const sidebarOverlay = document.getElementById('sidebar-overlay');
    
    if (mobileMenuToggle && sidebar && sidebarOverlay) {
        mobileMenuToggle.addEventListener('click', function() {
            sidebar.classList.toggle('open');
            sidebarOverlay.classList.toggle('open');
        });
        
        sidebarOverlay.addEventListener('click', function() {
            sidebar.classList.remove('open');
            sidebarOverlay.classList.remove('open');
        });
    }
    
    function expandToCurrentFile() {
        const currentPath = window.location.pathname;
        
        const activeFileLink = Array.from(document.querySelectorAll('.file-link')).find(
            link => link.getAttribute('href') === currentPath
        );
        
        if (activeFileLink) {
            activeFileLink.classList.add('text-blue-500', 'font-medium');
            
            let parent = activeFileLink.closest('.directory');
            while (parent) {
                const folderContents = parent.querySelector('.folder-contents');
                if (folderContents) {
                    folderContents.classList.remove('hidden');
                    folderContents.classList.add('block');
                }
                
                const toggleIcon = parent.querySelector('.toggle-icon');
                if (toggleIcon) {
                    toggleIcon.classList.add('rotate-90');
                }
                
                parent = parent.parentElement.closest('.directory');
            }
            
            setTimeout(() => {
                activeFileLink.scrollIntoView({ block: 'center', behavior: 'smooth' });
            }, 300);
        }
    }
    
    expandToCurrentFile();
});